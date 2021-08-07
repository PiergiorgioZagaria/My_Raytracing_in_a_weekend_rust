use crate::materials::*;
use crate::mylib::*;
use crate::vecmath::*;
use rand::Rng;
use std::sync::Arc;

/// A struct to store important data of the intersection of the rays
pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
    material: Arc<dyn Material + Sync + Send>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: Arc<dyn Material + Sync + Send>) -> Self {
        Self {
            t,
            p,
            normal,
            material,
        }
    }
    pub fn scatter(&self, r_in: &Ray, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        self.material.scatter(r_in, self, attenuation, scattered)
    }
    pub fn get_t(&self) -> f32 {
        self.t
    }
    pub fn get_p(&self) -> Vec3 {
        self.p
    }
    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }
}

/// A trait implemented by things that can be hit by a ray
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

/// A HitableList contains a list of all the objects that can be hit
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable + Sync>>,
}

impl HitableList {
    /// Checks wether the ray hit something in the list
    pub fn hit_list(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in self.list.iter() {
            if i.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        return hit_anything;
    }

    pub fn random_scene() -> Self {
        let n = 500;
        let mut list: Vec<Box<dyn Hitable + Sync>> = Vec::with_capacity(n + 1);
        list.push(Box::new(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let mat = rand::thread_rng().gen::<f32>();
                let center = Vec3::new(
                    a as f32 + 0.9 * rand::thread_rng().gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rand::thread_rng().gen::<f32>(),
                );
                if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                    if mat < 0.8 {
                        list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Lambertian::new(Vec3::new(
                                rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<f32>(),
                                rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<f32>(),
                                rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<f32>(),
                            ))),
                        )));
                    } else if mat < 0.95 {
                        list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Metal::new(
                                Vec3::new(
                                    0.5 * (1. + rand::thread_rng().gen::<f32>()),
                                    0.5 * (1. + rand::thread_rng().gen::<f32>()),
                                    0.5 * (1. + rand::thread_rng().gen::<f32>()),
                                ),
                                0.5 * rand::thread_rng().gen::<f32>(),
                            )),
                        )));
                    } else {
                        list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dieletric::new(1.5)),
                        )));
                    }
                }
            }
        }
        list.push(Box::new(Sphere::new(
            Vec3::new(0., 1., 0.),
            1.,
            Arc::new(Dieletric::new(1.5)),
        )));
        list.push(Box::new(Sphere::new(
            Vec3::new(-4., 1., 0.),
            1.,
            Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        list.push(Box::new(Sphere::new(
            Vec3::new(4., 1., 0.),
            1.,
            Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)),
        )));
        return Self { list };
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material + Sync + Send>) -> Self {
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
        let oc = r.get_origin() - self.center;
        let a = r.get_direction().dot(r.get_direction());
        let b = oc.dot(r.get_direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = b * b - a * c;
        if delta > 0. {
            let temp = (-b - delta.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-b + delta.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
            return false;
        } else {
            return false;
        }
    }
}
