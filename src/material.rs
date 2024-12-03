use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{self, Vec3};
use crate::common;
 
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
 
pub struct Lambertian {
    albedo: Color,
}
 
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo, }
    }
}
 
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + vec3::random_unit_vector();
 
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
 
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }
}
 
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
 
impl Metal {
    pub fn new(albedo: Color, f: f64) -> Metal {
        Metal { 
            albedo,
            fuzz: if f < 1.0 {
                f 
            } else {
                1.0 
            }, 
        }
    }
}
 
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected: Vec3 = vec3::reflect(r_in.direction(), rec.normal);
        reflected = vec3::unit_vector(reflected) + (self.fuzz * vec3::random_unit_vector());
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected);
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64, // Index of refraction
}
 
impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index, }
    }

     fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
     // Use Shlick's approximation for reflectance.
     let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
     r0 = r0 * r0;
     r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0) 
    }
}
 
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
 
        let unit_direction: Vec3 = vec3::unit_vector(r_in.direction());
        let cos_theta: f64 = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta: f64 = f64::sqrt(1.0 - cos_theta * cos_theta);
 
        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract || Self::reflectance(cos_theta, ri) > common::random_double() {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, ri)
        };

        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, direction);
        true
    }
}