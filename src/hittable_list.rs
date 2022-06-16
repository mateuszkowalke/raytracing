use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::point3;
use std::vec::Vec;

#[derive(Debug)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{
            objects: Vec::new(),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // TODO
        // this might be simplified, but left for readability for now
        let oc = &ray.orig.sub(self.center);
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(d);
        let mut nearest_root = (-b - sqrtd) / 2.0 * a;
        if nearest_root < t_min || nearest_root > t_max {
            nearest_root = (-b + sqrtd) / 2.0 * a;
            if nearest_root < t_min || nearest_root > t_max {
                return false;
            }
        }
        rec.t = nearest_root;
        rec.p =ray.at(nearest_root);
        rec.normal = rec.p.sub(self.center).div(self.radius);
        return true;
    }
}
