use crate::models::Model;
use crate::ray::{HitRecord, Ray};

pub struct World<'a> {
    objects: Vec<&'a dyn Model>,
    t_min: f64,
    t_max: f64,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
            t_min: 0.001,
            t_max: std::f64::INFINITY,
        }
    }

    pub fn add_object(&mut self, object: &'a dyn Model) {
        self.objects.push(object);
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
