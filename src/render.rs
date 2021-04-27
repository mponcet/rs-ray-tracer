use crate::camera::Camera;
use crate::config;
use crate::ray::Ray;
use crate::vec3::*;
use crate::world::World;
use rand::Rng;

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

pub fn render(world: &World, camera: &Camera) -> Vec<Color> {
    let mut pixels: Vec<Color> = Vec::with_capacity(config::IMAGE_WIDTH * config::IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();

    for j in (0..=config::IMAGE_HEIGHT).rev() {
        for i in 0..config::IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..config::SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..=1.0)) / (config::IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..=1.0)) / (config::IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, config::MAX_DEPTH);
            }

            let scale = 1.0 / config::SAMPLES_PER_PIXEL as f64;
            let color = Color::new(
                (scale * color.x()).sqrt(),
                (scale * color.y()).sqrt(),
                (scale * color.z()).sqrt(),
            );
            pixels.push(color);
        }
    }

    pixels
}
