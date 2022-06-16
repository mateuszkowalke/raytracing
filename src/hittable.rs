use crate::vec3::point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use std::fmt;

pub struct HitRecord {
    pub p: point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable: fmt::Debug {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
