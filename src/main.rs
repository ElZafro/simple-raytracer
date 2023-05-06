mod camera;
mod hit;
mod material;
mod ray;
mod sphere;

use std::sync::Arc;

use hit::{Hit, World};
use image::RgbImage;
use material::{generate_random_material, Dielectric, Lambertian, Metal};
use nalgebra::Vector3;
use rand::random;
use ray::Ray;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::*;
use sphere::Sphere;

use camera::Camera;

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

fn random_scene() -> World {
    let mut world = World::new();

    let material_ground = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    let sphere_ground = Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    world.push(Box::new(sphere_ground));

    for z in -11..11 {
        for x in -11..11 {
            let sphere = Sphere::new(
                Vector3::new(x as f64 + random::<f64>(), 0.2, z as f64 + random::<f64>()),
                0.2,
                generate_random_material(),
            );
            world.push(Box::new(sphere));
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: u64 = 50;

    let camera = Camera::new(
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
    );

    let world = random_scene();

    let image_buffer = (0..HEIGHT)
        .into_par_iter()
        .map(|y| {
            let y = HEIGHT - y;
            let mut row = [[0u8; 3]; WIDTH as usize];
            for (x, pixel) in row.iter_mut().enumerate() {
                let mut pixel_color = Vector3::<f64>::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (x as f64 + random::<f64>() - 0.5) / ((WIDTH - 1) as f64);
                    let v = (y as f64 + random::<f64>() - 0.5) / ((HEIGHT - 1) as f64);

                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                }
                pixel_color /= SAMPLES_PER_PIXEL as f64;
                *pixel = get_color(pixel_color);
            }
            println!("{}", y);
            row
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    let image_buffer = RgbImage::from_raw(WIDTH, HEIGHT, image_buffer).unwrap();
    image_buffer.save("image.png").unwrap();
}
