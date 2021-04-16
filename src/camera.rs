use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    look_from: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

use super::ASPECT_RATIO;

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let look_from = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            look_from - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            look_from,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.look_from,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.look_from,
        )
    }
}
