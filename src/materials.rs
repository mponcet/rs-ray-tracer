use rand::random;

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

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(ray.direction.unit_vector(), hitrec.normal);
        let scattered = Ray::new(
            hitrec.point,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );

        if scattered.direction.dot(hitrec.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

fn refract(uv: Vec3, n: Vec3, eta_ratio: f64) -> Vec3 {
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * eta_ratio;
    let r_out_parallel = n * -(1.0 - r_out_perp.len_squared()).abs().sqrt();

    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
    // schlick's approximation for reflectance
    let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if hitrec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(hitrec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let can_reflect = reflectance(cos_theta, refraction_ratio) > random();
        let direction = if cannot_refract || can_reflect {
            reflect(unit_direction, hitrec.normal)
        } else {
            refract(unit_direction, hitrec.normal, refraction_ratio)
        };

        Some(Scatter {
            ray: Ray::new(hitrec.point, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
