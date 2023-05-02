use std::f64::consts::PI;

use nalgebra::Vector3;

use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn new(
        origin: Vector3<f64>,
        look_at: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (origin - look_at).normalize();
        let cu = vup.cross(&cw).normalize();
        let cv = cw.cross(&cu);

        let horizontal = viewport_width * cu;
        let vertical = viewport_height * cv;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - cw;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
