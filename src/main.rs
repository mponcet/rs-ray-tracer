use std::fmt;

mod camera;
mod models;
mod ray;
mod vec3;

use crate::camera::Camera;
use crate::models::Sphere;
use crate::vec3::{Color, Point3};

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

fn main() {
    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let camera = Camera::new(viewport_width, viewport_height);

    // Sphere
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

    let mut pixels: Vec<Color> = Vec::with_capacity(IMAGE_WIDTH * IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = camera.get_ray(u, v);



            let color = if let Some(t) = sphere.hit(&ray) {
                let normal = (ray.point_at(t) - Point3::new(0.0, 0.0, -1.0)).unit_vector();
                Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5
            } else {
                let unit_direction = ray.direction.unit_vector();
                let t = 0.5 * unit_direction.y() + 1.0;
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            };

            pixels.push(color);
        }
    }

    let image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, pixels);
    println!("{}", image);
}
