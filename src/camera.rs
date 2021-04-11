use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    look_from: Point3,
    viewport_width: f64,
    viewport_height: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

const FOCAL_LENGTH: f64 = 1.0;

impl Camera {
    pub fn new(viewport_width: f64, viewport_height: f64) -> Self {
        let look_from = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            look_from - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            look_from,
            viewport_width,
            viewport_height,
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
