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

pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let n = if rec.front_face { rec.normal } else { rec.normal * -1.0 };

        let cos_theta = (-1.0 * unit_direction).dot(&n).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = random::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            reflect(&unit_direction, &n)
        } else {
            refract(&unit_direction, &n, refraction_ratio)
        };

        let scattered = Ray::new(rec.point, direction);

        Some((scattered, Vector3::new(1.0, 1.0, 1.0)))
    }
}