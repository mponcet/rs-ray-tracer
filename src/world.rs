use crate::materials::*;
use crate::models::{Model, Sphere};
use crate::ray::{HitRecord, Ray};
use crate::vec3::{Color, Point3};

use itertools::Itertools;
use rand::Rng;

pub struct World {
    objects: Vec<Box<dyn Model>>,
    t_min: f64,
    t_max: f64,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
            t_min: 0.001,
            t_max: std::f64::INFINITY,
        }
    }

    pub fn add_object(&mut self, object: impl Model + 'static) {
        self.objects.push(Box::new(object));
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

    pub fn random() -> Self {
        let mut world = Self::new();
        let mut rng = rand::thread_rng();

        world.add_object(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            },
        ));

        world.add_object(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric {
                refraction_index: 1.5,
            },
        ));

        world.add_object(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Lambertian {
                albedo: Color::new(0.4, 0.2, 0.1),
            },
        ));

        world.add_object(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Metal {
                albedo: Color::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            },
        ));

        for (x, z) in (-10..6).cartesian_product(-10..10) {
            if x == 4 || x == 0 || x == -4 {
                continue;
            }

            let radius = rng.gen_range(0.1..0.3);
            let p = Point3::new(
                x as f64 + rng.gen_range(0.0..0.5),
                radius,
                z as f64 + rng.gen_range(0.0..0.5),
            );
            match rand::random::<u8>() % 5 {
                0 => {
                    world.add_object(Sphere::new(
                        p,
                        radius,
                        Lambertian {
                            albedo: Color::random(0.0, 1.0),
                        },
                    ));
                }
                1 => {
                    world.add_object(Sphere::new(
                        p,
                        radius,
                        Dielectric {
                            refraction_index: 1.5,
                        },
                    ));
                }
                2 => {
                    world.add_object(Sphere::new(
                        p,
                        radius,
                        Metal {
                            albedo: Color::random(0.0, 1.0),
                            fuzz: rng.gen_range(0.0..0.5),
                        },
                    ));
                }
                _ => {}
            }
        }

        world
    }
}
