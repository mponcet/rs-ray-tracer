use crate::ray::{Ray, HitRecord};
use crate::models::Sphere;

// TODO: make objects generic
pub struct World {
    objects: Vec<Sphere>,
    t_min: f64,
    t_max: f64,
}

impl World {
    pub fn new() -> Self {
        World { objects: Vec::new(), t_min: 0., t_max: std::f64::INFINITY  }
    }

    pub fn add_object(&mut self, sphere: Sphere) {
        self.objects.push(sphere);
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest_so_far = self.t_max;
        let mut best_hitrec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hitrec) = object.hit(ray, self.t_min, closest_so_far) {
                closest_so_far = hitrec.t;
                best_hitrec = Some(hitrec);
            }
        }

        best_hitrec
    }
}
