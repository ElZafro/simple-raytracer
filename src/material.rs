use nalgebra::Vector3;
use rand::random;

use crate::{ray::{Ray, reflect, refract}, hit::HitRecord};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    albedo: Vector3<f64>
}

impl Lambertian {
    pub fn new(color: Vector3<f64>) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let random_unit_vector = Vector3::new(random::<f64>() * 2.0 - 1.0 , random::<f64>() * 2.0 - 1.0, random::<f64>() * 2.0 - 1.0).normalize();
        let mut scatter_direction = hit_record.normal + random_unit_vector;
        
        if scatter_direction.norm_squared() <= f64::EPSILON { scatter_direction = hit_record.normal; }
        
        let scattered_ray = Ray::new(hit_record.point, scatter_direction);
        Some((scattered_ray, self.albedo))
    }
}

pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64
}

impl Metal {
    pub fn new(color: Vector3<f64>, fuzz: f64) -> Self {
        Self { albedo: color, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        
        let reflected = reflect(&r_in.direction, &hit_record.normal).normalize();
        let random_unit_vector = Vector3::new(random::<f64>() * 2.0 - 1.0 , random::<f64>() * 2.0 - 1.0, random::<f64>() * 2.0 - 1.0).normalize();
        let scattered_ray = Ray::new(hit_record.point, reflected + self.fuzz * random_unit_vector);

        if reflected.dot(&hit_record.normal) <= f64::EPSILON { return None; }

        Some((scattered_ray, self.albedo))
    }
}