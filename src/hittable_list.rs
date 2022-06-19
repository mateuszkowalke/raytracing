use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use std::vec::Vec;

#[derive(Debug)]
pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: Box<dyn Hittable + 'a>) {
        self.objects.push(obj);
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut hit_anything = false;
        let mut closest = t_max;
        self.objects.iter().for_each(|object| {
            if let Some(rec) = object.hit(ray, t_min, closest) {
                hit_anything = true;
                closest = rec.t;
                temp_rec = Some(rec);
            }
        });
        temp_rec
    }
}
