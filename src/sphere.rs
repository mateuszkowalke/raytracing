use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // TODO
        // this might be simplified, but left for readability for now
        let oc = &ray.orig.sub(&self.center);
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return None;
        }

        let mut nearest_root = (-b - d.sqrt()) / (2.0 * a);
        if nearest_root < t_min || nearest_root > t_max {
            nearest_root = (-b + d.sqrt()) / (2.0 * a);
            if nearest_root < t_min || nearest_root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: nearest_root,
            p: ray.at(nearest_root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let outward_normal = (rec.p.sub(&self.center)).div(self.radius);
        rec.set_face_normal(ray, outward_normal);
        Some(rec)
    }
}
