use std::{f64::EPSILON, rc::Rc};

use nalgebra::Vector3;

use crate::{
    hit::{Hit, HitRecord},
    material::Scatter,
    ray::Ray,
};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Rc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius.powi(2);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let mut root = (-half_b - sqrt) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let n = (p - self.center) / self.radius;
        Some(HitRecord {
            point: p,
            normal: n,
            t: root,
            material: self.material.clone(),
            front_face: r.direction.dot(&n) <= EPSILON,
        })
    }
}
