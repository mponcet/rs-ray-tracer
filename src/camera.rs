use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    look_from: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = look_from - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        Self {
            look_from,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rnd_disk = Vec3::random_in_unit_disk() * self.lens_radius;
        let rnd_offset = self.u * rnd_disk.x() + self.v * rnd_disk.y();

        Ray::new(
            self.look_from + rnd_offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.look_from
                - rnd_offset,
        )
    }
}
