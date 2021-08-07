use crate::hitables::*;
use crate::mylib::*;
use crate::vecmath::Vec3;
use rand::Rng;

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
fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = v.unit_vector();
    let dt = uv.dot(*n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        *refracted = (uv - (*n) * dt) * ni_over_nt - (*n) * discriminant.sqrt();
        return true;
    } else {
        return false;
    }
}

// The reflectivity varies with the angle, think of a window and look
// from different sides. This is an approximation to take this into account
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0: f32 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * f32::powf(1. - cosine, 5.);
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let rec_p = rec.get_p();
        let rec_normal = rec.get_normal();
        let target = rec_p + rec_normal + Vec3::random_in_unit_sphere();
        *scattered = Ray::new(rec_p, target - rec_p);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f32) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.get_direction().unit_vector(), &rec.get_normal());
        *scattered = Ray::new(
            rec.get_p(),
            reflected
                + Vec3::random_in_unit_sphere()
                    * if self.fuzziness > 1. {
                        1.
                    } else {
                        self.fuzziness
                    },
        );
        *attenuation = self.albedo;
        return scattered.get_direction().dot(rec.get_normal()) > 0.;
    }
}

pub struct Dieletric {
    ref_idx: f32,
}

impl Dieletric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dieletric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal: Vec3;
        let reflected: Vec3 = reflect(&r_in.get_direction(), &rec.get_normal());
        let ni_over_nt: f32;
        *attenuation = Vec3::new(1., 1., 1.);
        let mut refracted: Vec3 = Vec3::new(0., 0., 0.);
        let reflect_prob: f32;
        let cosine: f32;
        if r_in.get_direction().dot(rec.get_normal()) > 0. {
            outward_normal = rec.get_normal() * (-1.);
            ni_over_nt = self.ref_idx;
            cosine = r_in.get_direction().dot(rec.get_normal()) * self.ref_idx
                / r_in.get_direction().length();
        } else {
            outward_normal = rec.get_normal();
            ni_over_nt = 1. / self.ref_idx;
            cosine =
                r_in.get_direction().dot(rec.get_normal()) / (-1. * r_in.get_direction().length());
        }

        if refract(
            &r_in.get_direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            *scattered = Ray::new(rec.get_p(), reflected);
            reflect_prob = 1.;
        }
        if rand::thread_rng().gen::<f32>() < reflect_prob {
            *scattered = Ray::new(rec.get_p(), reflected);
        } else {
            *scattered = Ray::new(rec.get_p(), refracted);
        }
        return true;
    }
}
