use crate::materials::*;
use crate::hitables::*;
use crate::vecmath::Vec3;
use rand::Rng;
use std::sync::Arc;

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

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }
}

impl Vec3 {
    // We need this for color_matte(), creates a random vector,
    // Inside the unit sphere tangent to the hitpoint
    pub fn random_in_unit_sphere() -> Self {
        let mut p: Vec3;
        let mut rng = rand::thread_rng();
        loop {
            p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.- Vec3::new(1., 1., 1.);
            // radius <= 1
            if p.squared_len() < 1. {
                return p;
            }
        }
    }

    // We need this to simulate the focus and blur of the image
    pub fn random_in_unit_disc() -> Self {
        let mut p: Vec3;
        let mut rng = rand::thread_rng();
        loop {
            p = Vec3::new(rng.gen(), rng.gen(), 0.) * 2. - Vec3::new(1., 1., 0.);
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
            Arc::new(Lambertian::new(Vec3::new(0., 0., 0.))),
        );
        // To prevent shadow acne, try setting it to other values
        if world.hit_list(&r, 0.001, f32::MAX, &mut rec) {
            let mut scattered = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
            let mut attenuation = Vec3::new(0., 0., 0.);
            if depth < 50 && rec.scatter(r, &mut attenuation, &mut scattered) {
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

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius = aperture / 2.;
        let theta = vfov * (std::f32::consts::PI) / 180.;
        let halfheight = (theta / 2.).tan();
        let halfwidth = aspect * halfheight;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);
        Self {
            origin,
            lower_left_corner: origin
                - u * halfwidth * focus_dist
                - v * halfheight * focus_dist
                - w * focus_dist,
            horizontal: u * 2. * halfwidth * focus_dist,
            vertical: v * 2. * halfheight * focus_dist,
            u,
            v,
            w,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

