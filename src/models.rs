use crate::materials::Material;
use crate::ray::{HitRecord, Ray};
use crate::vec3::Point3;

pub trait Model {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere<T: Material> {
    center: Point3,
    radius: f64,
    material: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Point3, radius: f64, material: T) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material> Model for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (-half_b - sqrtd) / a;
        if t < t_min || t > t_max {
            t = (-half_b + sqrtd) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.point_at(t);
        let normal = (point - self.center) / self.radius;
        let front_face = normal.dot(ray.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Some(HitRecord {
            point,
            normal,
            front_face,
            t,
            material: &self.material,
        })
    }
}
