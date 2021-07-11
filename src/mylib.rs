use crate::vecmath::Vec3;
use rand::Rng;

/// A ray can be seen as a funtion p(t) = a + t * b
/// Where 'a' is the origin and 'b' is the direction
/// In the end it is just a straight line, and p(t)
/// Is the position of the ray at time t
#[derive(Debug)]
pub struct Ray{
    origin: Vec3,
    direction: Vec3,
}

impl Ray{
    /// Create a new ray given 'a' starting position and 'b' direction
    pub fn new(origin: Vec3, direction: Vec3) -> Self{
        Self{origin,direction}
    }
    /// Position at certain 't' parameter
    pub fn point_at_parameter(&self,t: f32) -> Vec3{
        self.origin + self.direction * t
    }
    pub fn get_origin(&self) -> Vec3 {
        return self.origin;
    }
    pub fn get_direction(&self) -> Vec3 {
        return self.direction;
    }
}

impl Vec3 {
    // We need this for color_matte(), creates a random vector,
    // Inside the unit sphere tangent to the hitpoint
    fn random_in_unit_sphere() -> Self{
        let mut p:Vec3;
        let mut rng = rand::thread_rng();
        loop{
            p = Vec3::new(rng.gen(),rng.gen(),rng.gen()) * 2.;
            // radius <= 1
            if p.squared_len() < 1. {return p;}
        }
    }

    /// To use on matte (diffuse) surfaces. Diffuse surfaces make the 
    /// ray bounce around at random, so they absorb the color of the 
    /// things around them, try changing the colors in the else.
    pub fn color_matte(r: Ray,world: &HitableList) -> Self{
        let mut rec: HitRecord = HitRecord::new(0.,Vec3::new(0.,0.,0.), Vec3::new(0.,0.,0.));
        // The 0.001 is so that we can prevent shadow acne,
        // try to set it to zero and notice differences, in
        // paticular at the top of the big sphere under the 
        // small one
        if world.hit_list(&r,0.001,f32::MAX,&mut rec) {
            // Make the ray bounce around.
            // First it creates a random direction with random_in_unit_sphere,
            // then it calls color with the bounced ray, until it can't hit anything else
            let target = rec.get_p() + rec.get_normal() + Vec3::random_in_unit_sphere();
            return Vec3::color_matte(Ray::new(rec.p,target - rec.get_p()),world) * 0.5;
        }else{
            // Linearly blend blue and white based on the direction y coordinate
            // High y = blue, Low y = white. This is called linear interpolation
            let unit = r.get_direction().unit_vector();
            let t = (unit.y + 1.) * 0.5;
            Vec3::new(1.,1.,1.)*(1. - t) + Vec3::new(1.,0.4,0.65)*t
        }
    }

    // Color returned based on the normal from the intersection of the ray and the sphere
    pub fn color(r: Ray,world: &HitableList) -> Self{
        let mut rec: HitRecord = HitRecord::new(0.,Vec3::new(0.,0.,0.), Vec3::new(0.,0.,0.));
        if world.hit_list(&r,0.,f32::MAX,&mut rec) {
            // Return the color created from the normal's coordinates
            return (rec.get_normal() + Vec3::new(1.,1.,1.)) * 0.5;
        }else{
            // Linearly blend blue and white based on the direction y coordinate
            // High y = blue, Low y = white. This is called linear interpolation
            let unit = r.get_direction().unit_vector();
            let t = (unit.y + 1.) * 0.5;
            Vec3::new(1.,1.,1.)*(1. - t) + Vec3::new(0.5,0.7,1.)*t
        }
    }
}

/// Transform a color vector in u32
impl From<Vec3> for u32 {
    fn from(t: Vec3) -> Self{
        let r = t.x as u32;
        let g = t.y as u32;
        let b = t.z as u32;
        r << 16 | g << 8 | b
    }
}

// Maybe there is a way to change this as it is quite slow, but I'm not sure, It is like this because I need it later for HitableList hit()
#[derive(Clone,Copy)]
pub struct HitRecord{
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord{
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> Self {
        Self {t,p,normal}
    }
    pub fn get_t(&self) -> f32{
        return self.t;
    }
    pub fn get_p(&self) -> Vec3{
        return self.p;
    }
    pub fn get_normal(&self) -> Vec3{
        return self.normal;
    }
}

/// A trait implemented by things that can be hit by a ray
pub trait Hitable{
    fn hit(&self,r: &Ray,t_min: f32, t_max: f32,rec: &mut HitRecord) -> bool;
    fn up(&mut self);
}

pub struct Sphere{
    center: Vec3,
    radius: f32,
}

impl Sphere{
    pub fn new(center: Vec3,radius: f32) -> Self{
        Self{center,radius}
    }
}

impl Hitable for Sphere{
    /// Returns true if the ray hit the sphere and if it does, sets
    /// t_min as the closest value to the origin, so we will see 
    /// what is directly in front of us and not behind, the HitRecord
    /// stores the distance t, the point of intersection and the normal
    /// of the object
    fn hit(&self,r: &Ray,t_min:f32,t_max:f32,rec:&mut HitRecord) -> bool{
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = b * b -  a * c;
        if delta > 0.{
            let temp = (-b - delta.sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center)/self.radius;
                return true;
            }
            let temp = (-b + delta.sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center)/self.radius;
                return true;
            }
            return false;
        }else{
            return false;
        }
    }

    /// Used just to test things
    fn up(&mut self){
        self.center.y += 1.;
    }
}

/// A HitableList contains a list of all the objects that can be hit
pub struct HitableList{
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    /// Checks wether the ray hit something in the list
    pub fn hit_list(&self,r: &Ray, t_min:f32,t_max:f32,rec:&mut HitRecord) -> bool{
        let mut temp_rec: HitRecord = HitRecord {t: 0.,p:Vec3::new(0.,0.,0.),normal: Vec3::new(0.,0.,0.)};
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in self.list.iter(){
            if i.hit(r,t_min,closest_so_far,&mut temp_rec) {
                *rec = temp_rec;
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        return hit_anything;
    }
}

pub struct Camera{
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera{
    pub fn new(origin:Vec3,lower_left_corner:Vec3,horizontal:Vec3,vertical:Vec3) -> Self{
        Self{origin,lower_left_corner,horizontal,vertical}
    }
    pub fn get_ray(&self,u:f32,v:f32) -> Ray {
        Ray::new(self.origin,self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
