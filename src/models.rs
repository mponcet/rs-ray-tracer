use crate::materials::Material;
use crate::ray::{HitRecord, Ray};
use crate::vec3::Point3;

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a dyn Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let t = (-half_b - sqrtd) / a;
        if t > t_min && t < t_max {
            let point = ray.point_at(t);
            return Some(HitRecord {
                point,
                normal: (point - self.center) / self.radius,
                t,
                material: self.material,
            });
        }

        let t = (-half_b + sqrtd) / a;
        if t > t_min && t < t_max {
            let point = ray.point_at(t);
            return Some(HitRecord {
                point,
                normal: (point - self.center) / self.radius,
                t,
                material: self.material,
            });
        }

        None
    }
}
