use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::interval::Interval;
 
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
 
impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
 
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
 
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;
 
        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
 
        hit_anything
    }
}