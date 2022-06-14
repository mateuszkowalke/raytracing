use super::vec3;

#[derive(Debug)]
pub struct Ray<'a> {
    pub orig: &'a vec3::point3,
    pub dir: &'a vec3::Vec3,
}

impl <'a> Ray<'a> {
    pub fn new(orig: &'a vec3::point3, dir: &'a vec3::Vec3) -> Ray<'a> {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> vec3::point3 {
        self.orig.add(&self.dir.multiply(t))
    }
}
