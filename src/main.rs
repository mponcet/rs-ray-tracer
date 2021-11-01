use std::fmt;

mod camera;
mod config;
mod materials;
mod models;
mod ray;
mod render;
mod vec3;
mod world;

use crate::camera::Camera;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::World;

struct PPMImage {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl PPMImage {
    fn new(width: usize, height: usize, pixels: Vec<Vec<Color>>) -> Self {
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
        for pixel_j in &self.pixels {
            for pixel in pixel_j {
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
        }

        write!(f, "\n")
    }
}

fn main() {
    // World
    let world = World::random();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        config::ASPECT_RATIO,
        0.1,
        10.0,
    );

    let pixels = render::render(&world, &camera);

    let image = PPMImage::new(config::IMAGE_WIDTH, config::IMAGE_HEIGHT, pixels);
    println!("{}", image);
}
