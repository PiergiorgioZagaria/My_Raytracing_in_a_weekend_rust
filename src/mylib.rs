use crate::vecmath::Vec3;
use rand::Rng;

/// A ray can be seen as a funtion p(t) = a + t * b
/// Where 'a' is the origin and 'b' is the direction
/// In the end it is just a straight line, and p(t)
/// Is the position of the ray at time t
#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    /// Create a new ray given 'a' starting position and 'b' direction
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    /// Position at certain 't' parameter
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl Vec3 {
    // We need this for color_matte(), creates a random vector,
    // Inside the unit sphere tangent to the hitpoint
    fn random_in_unit_sphere() -> Self {
        let mut p: Vec3;
        let mut rng = rand::thread_rng();
        loop {
            p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.;
            // radius <= 1
            if p.squared_len() < 1. {
                return p;
            }
        }
    }
    //     // Color returned based on the normal from the intersection of the ray and the sphere
    //     pub fn color(r: Ray,world: &HitableList) -> Self{
    //         let mut rec: HitRecord = HitRecord::new(0.,Vec3::new(0.,0.,0.), Vec3::new(0.,0.,0.));
    //         if world.hit_list(&r,0.,f32::MAX,&mut rec) {
    //             // Return the color created from the normal's coordinates
    //             return (rec.normal + Vec3::new(1.,1.,1.)) * 0.5;
    //         }else{
    //             // Linearly blend blue and white based on the direction y coordinate
    //             // High y = blue, Low y = white. This is called linear interpolation
    //             let unit = r.direction.unit_vector();
    //             let t = (unit.y + 1.) * 0.5;
    //             Vec3::new(1.,1.,1.)*(1. - t) + Vec3::new(0.5,0.7,1.)*t
    //         }
    //     }

    pub fn color_material(r: &Ray, world: &HitableList, depth: i32) -> Vec3 {
        let mut rec: HitRecord = HitRecord::new(
            0.,
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 0., 0.),
            Materials::Lambertian(Vec3::new(0., 0., 0.)),
        );
        // To prevent shadow acne, try setting it to other values
        if world.hit_list(&r, 0.001, f32::MAX, &mut rec) {
            let mut scattered = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
            let mut attenuation = Vec3::new(0., 0., 0.);
            if depth < 50 && HitRecord::scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Vec3::color_material(&scattered, world, depth + 1);
            } else {
                return Vec3::new(0., 0., 0.);
            }
        } else {
            // Linearly blend blue and white based on the direction y coordinate
            // High y = blue, Low y = white. This is called linear interpolation
            let unit = r.direction.unit_vector();
            let t = (unit.y + 1.) * 0.5;
            Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
}

/// Transform a color vector in u32
impl From<Vec3> for u32 {
    fn from(t: Vec3) -> Self {
        let r = t.x as u32;
        let g = t.y as u32;
        let b = t.z as u32;
        r << 16 | g << 8 | b
    }
}

// Maybe there is a way to change this as it is quite slow, but I'm not sure, It is like this because I need it later for HitableList hit()
#[derive(Clone, Copy)]
pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
    material: Materials,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: Materials) -> Self {
        Self {
            t,
            p,
            normal,
            material,
        }
    }
}

/// A trait implemented by things that can be hit by a ray
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Materials,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Materials) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    /// Returns true if the ray hit the sphere and if it does, sets
    /// t_min as the closest value to the origin, so we will see
    /// what is directly in front of us and not behind, the HitRecord
    /// stores the distance t, the point of intersection and the normal
    /// of the object
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = b * b - a * c;
        if delta > 0. {
            let temp = (-b - delta.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;
                return true;
            }
            let temp = (-b + delta.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;
                return true;
            }
            return false;
        } else {
            return false;
        }
    }
}

/// A HitableList contains a list of all the objects that can be hit
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    /// Checks wether the ray hit something in the list
    pub fn hit_list(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new(
            0.,
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 0., 0.),
            Materials::Lambertian(Vec3::new(0., 0., 0.)),
        );
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in self.list.iter() {
            if i.hit(r, t_min, closest_so_far, &mut temp_rec) {
                *rec = temp_rec;
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        return hit_anything;
    }
}

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Self {
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

// The direction after a ray has been reflected off
// a metal surface
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - (*n) * v.dot(*n) * 2.;
}

// The direction after a ray has passed through a 
// a glass or transparent object
// TODO consider that the function return false even if
// there is a reflection ray and not a refraction one,
// TODO correct this at some point
fn refract(v: &Vec3,n: &Vec3,ni_over_nt: f32,refracted: &mut Vec3)->bool{
    let uv = v.unit_vector();
    let dt = uv.dot(*n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        *refracted = (uv - (*n) * dt) * ni_over_nt - (*n) * discriminant.sqrt();
        return true;
    }else{
        return false;
    }
}

// The reflectivity varies with the angle, think of a window and look
// from different sides. This is an approximation to take this into account
fn schlick(cosine: f32, ref_idx: f32) -> f32{
    let mut r0: f32 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * f32::powf(1. - cosine,5.);
}

#[derive(Clone, Copy)]
pub enum Materials {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dieletric(f32),
}

// TODO Maybe use Material as a trait for ducktyping? Don't know how though
// pub trait Mat {
//     fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
// }

impl HitRecord {
    pub fn scatter(
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match rec.material {
            // To use on matte (diffuse, Lambertian) surfaces. Diffuse surfaces make the
            // ray bounce around at random, so they absorb the color of the
            // things around them, try changing the colors in the else.
            Materials::Lambertian(v) => {
                let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
                *scattered = Ray::new(rec.p, target - rec.p);
                *attenuation = v;
                return true;
            }
            Materials::Metal(v, f) => {
                let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);
                *scattered = Ray::new(
                    rec.p,
                    reflected + Vec3::random_in_unit_sphere() * if f > 1. { 1. } else { f },
                );
                *attenuation = v;
                return scattered.direction.dot(rec.normal) > 0.;
            }
            Materials::Dieletric(ref_idx) => {
                let outward_normal:Vec3;
                let reflected:Vec3 = reflect(&r_in.direction,&rec.normal);
                let ni_over_nt:f32;
                *attenuation = Vec3::new(1.,1.,1.);
                let mut refracted:Vec3 = Vec3::new(0.,0.,0.);
                let reflect_prob:f32;
                let cosine:f32;
                if r_in.direction.dot(rec.normal) > 0. {
                    outward_normal = rec.normal * (-1.);
                    ni_over_nt = ref_idx;
                    cosine = r_in.direction.dot(rec.normal) * ref_idx / r_in.direction.length();
                }else{
                    outward_normal = rec.normal;
                    ni_over_nt = 1. / ref_idx;
                    cosine = r_in.direction.dot(rec.normal) / (-1. * r_in.direction.length())
                }

                if refract(&r_in.direction,&outward_normal,ni_over_nt,&mut refracted){
                    reflect_prob = schlick(cosine,ref_idx);
                    // *scattered = Ray::new(rec.p,refracted);
                    // return true;
                }else{
                    *scattered = Ray::new(rec.p,reflected);
                    reflect_prob = 1.;
                    // return false;
                }
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    *scattered = Ray::new(rec.p,reflected);
                }else{
                    *scattered = Ray::new(rec.p,refracted);
                }
                return true;
            }
            _ => return false,
        }
    }
}
