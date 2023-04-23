use nalgebra::Vector3;

use crate::ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
  pub point: Vector3<f64>,
  pub normal: Vector3<f64>,
  pub t: f64
}

pub type World = Vec<Box<dyn Hit>>;

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter()
            .map(|el| el.hit(r, t_min, t_max))
            .filter(|opt| opt.is_some())
            .map(|el| el.unwrap())
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}