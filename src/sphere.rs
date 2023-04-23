use nalgebra::Vector3;

use crate::{hit::{Hit, HitRecord}, ray::Ray};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64
}

impl Sphere{
    pub fn new(center: Vector3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius.powi(2);
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0  { return None; }

        let sqrt = discriminant.sqrt();
        let mut root = (- half_b - sqrt) / a;

        if root < t_min || t_max < root {
            root = (- half_b + sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        Some( 
            HitRecord {
                point: p,
                normal: (p - self.center) / self.radius,
                t: root,
            }
        )
    }
}