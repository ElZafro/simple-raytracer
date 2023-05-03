mod camera;
mod hit;
mod material;
mod ray;
mod sphere;

use std::sync::Arc;

use hit::{Hit, World};
use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::Vector3;
use ray::Ray;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::*;
use sphere::Sphere;

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Vector3<f64> {
    if let Some(hit_record) = world.hit(r, 0.001, f64::MAX) {
        if depth == 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        if let Some((scattered_ray, attenuation)) = hit_record.material.scatter(r, &hit_record) {
            return attenuation.component_mul(&ray_color(&scattered_ray, world, depth - 1));
        }
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn get_color(pixel_color: Vector3<f64>) -> [u8; 3] {
    [
        (256.0 * (pixel_color.x).sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * (pixel_color.y).sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * (pixel_color.z).sqrt().clamp(0.0, 0.999)) as u8,
    ]
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 300;
    const MAX_DEPTH: u64 = 5;

    //World
    let mut world = World::new();
    let material_ground = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let sphere_left_inner = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.4, material_left);
    let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_left_inner));
    world.push(Box::new(sphere_right));

    let camera = Camera::new(
        Vector3::new(0.3, 0.84, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        55.0,
        16.0 / 9.0,
    );

    let image_buffer = (0..HEIGHT)
        .into_par_iter()
        .map(|y| {
            let mut row = [[0u8; 3]; WIDTH as usize];
            for x in 0..(WIDTH as usize) {
                let mut pixel_color = Vector3::<f64>::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let y = HEIGHT - y;

                    let u = (x as f64 + rand::random::<f64>() - 0.5) / ((WIDTH - 1) as f64);
                    let v = (y as f64 + rand::random::<f64>() - 0.5) / ((HEIGHT - 1) as f64);

                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                }
                pixel_color /= SAMPLES_PER_PIXEL as f64;
                row[x] = get_color(pixel_color);
            }
            row
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    let image_buffer = RgbImage::from_raw(WIDTH, HEIGHT, image_buffer).unwrap();
    image_buffer.save("image.png").unwrap();
}
