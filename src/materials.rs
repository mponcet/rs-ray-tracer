use crate::ray::{HitRecord, Ray};
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hitrec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hitrec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hitrec.normal;
        }

        Some(Scatter {
            ray: Ray::new(hitrec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
