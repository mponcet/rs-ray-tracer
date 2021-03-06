use crate::materials::Material;
use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64, // point = A + tb
    pub material: &'a dyn Material,
}
