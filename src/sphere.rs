use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Point3};
use crate::interval::Interval;
use crate::material::Material;
 
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>
}
 
impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere { 
            center,
            radius,
            mat,
        }
    }
}
 
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Point3 = self.center - r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = vec3::dot(r.direction(), oc);
        let c: f64 = oc.length_squared() - self.radius*self.radius;
        
        let discriminant: f64 = h*h - a*c;
        if discriminant < 0.0 {
            return false;
        }
 
        let sqrtd: f64 = f64::sqrt(discriminant);
 
        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
 
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        
        true
    }
}