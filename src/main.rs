mod ray;
mod hit;
mod sphere;
mod material;

use std::rc::Rc;

use hit::{Hit, World};
use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;
use ray::Ray;
use sphere::Sphere;

use crate::material::{Lambertian, Metal};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Vector3<f64> {

    if depth == 0 { return Vector3::new(0.0, 0.0, 0.0); }

    if let Some(hit_record) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered_ray, attenuation)) = hit_record.material.scatter(r, &hit_record) {
            return attenuation.component_mul(&ray_color(&scattered_ray, world, depth - 1));
        }
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn get_color(pixel_color: Vector3<f64>) -> Rgb<u8> {
    Rgb([
        (256.0 * (pixel_color.x).sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * (pixel_color.y).sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * (pixel_color.z).sqrt().clamp(0.0, 0.999)) as u8,
    ])
}

fn main() {

    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: usize = 256;
    const HEIGHT: usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 300;
    const MAX_DEPTH: u64 = 5;

    //World
    let mut world = World::new();
    let material_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.0));
    let material_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    //Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin 
        - horizontal / 2.0 
        - vertical / 2.0 
        - Vector3::new(0.0, 0.0, focal_length);


    let mut imgbuf = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut pixel_color = Vector3::<f64>::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let y = HEIGHT as u32 - y;

            let u = (x as f64 + rand::random::<f64>() - 0.5) / ((WIDTH - 1) as f64);
            let v = (y as f64 + rand::random::<f64>() - 0.5) / ((HEIGHT - 1) as f64);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);
            pixel_color += ray_color(&ray, &world, MAX_DEPTH);
        }
        pixel_color /= SAMPLES_PER_PIXEL as f64;
        *pixel = get_color(pixel_color);
    }

    imgbuf.save("image.png").unwrap();
    
}
