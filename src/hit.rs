use std::sync::Arc;

use nalgebra::Vector3;

use crate::{material::Scatter, ray::Ray};

pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub material: Arc<dyn Scatter>,
    pub front_face: bool,
}

pub type World = Vec<Box<dyn Hit>>;

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter()
            .filter_map(|el| el.hit(r, t_min, t_max))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
