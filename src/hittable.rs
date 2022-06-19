use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::fmt;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.multiply(-1.0)
        };
    }
}

pub trait Hittable: fmt::Debug {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
