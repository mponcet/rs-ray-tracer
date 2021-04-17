use rand::Rng;
use std::fmt;

mod camera;
mod materials;
mod models;
mod ray;
mod vec3;
mod world;

use crate::camera::Camera;
use crate::materials::{Dielectric, Lambertian, Metal};
use crate::models::Sphere;
use crate::ray::Ray;
use crate::vec3::{Color, Point3};
use crate::world::World;

struct PPMImage {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl PPMImage {
    fn new(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl fmt::Display for PPMImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height)?;

        let mut count = 0;
        for pixel in &self.pixels {
            let rgb = pixel.to_rgb();
            write!(f, "{} {} {}", rgb.0, rgb.1, rgb.2).unwrap();
            if count >= 6 {
                write!(f, "\n")?;
                count = 0;
            } else {
                write!(f, " ")?;
                count += 1;
            }
        }

        write!(f, "\n")
    }
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 800;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: usize = 50;

fn ray_color(ray: &Ray, world: &World, depth: usize) -> Color {
    if depth <= 0 {
        Color::new(0.0, 0.0, 0.0)
    } else if let Some(hitrec) = world.hit(ray) {
        if let Some(scatterd) = hitrec.material.scatter(ray, &hitrec) {
            return ray_color(&scatterd.ray, world, depth - 1) * scatterd.attenuation;
        }
        Color::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // World
    let mut world = World::new();
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Dielectric {
        refraction_index: 1.5,
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };
    let spheres = vec![
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground),
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &material_center),
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, &material_left),
        Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, &material_right),
    ];

    spheres.iter().for_each(|sphere| world.add_object(sphere));

    // Camera
    let camera = Camera::new();

    let mut pixels: Vec<Color> = Vec::with_capacity(IMAGE_WIDTH * IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..=1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..=1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, MAX_DEPTH);
            }

            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
            let color = Color::new(
                (scale * color.x()).sqrt(),
                (scale * color.y()).sqrt(),
                (scale * color.z()).sqrt(),
            );
            pixels.push(color);
        }
    }

    let image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, pixels);
    println!("{}", image);
}
