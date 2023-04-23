use nalgebra::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction 
    }
}

pub fn reflect(ray: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    ray - 2.0 * ray.dot(&normal) * normal
}

pub fn refract(ray: &Vector3<f64>, normal: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = ((-1.0) * ray).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (ray + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}