use nalgebra_glm::DVec3;
use nalgebra_glm as glm;
use serde::{Serialize, Deserialize};

use crate::hit::Ray;

use super::Material;
use rand::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Dielectric {
    /// index of refraction
    pub eta: f64
}

impl Dielectric {
    pub fn new(eta: f64) -> Self {
        Self { eta }
    }
}

#[typetag::serde]
impl Material for Dielectric {
    fn scatter(&self, ray: &crate::hit::Ray, hit: &crate::hit::HitRecord) -> Vec<(nalgebra_glm::DVec3, crate::hit::Ray)> {
        let cos_theta = (-ray.direction).normalize().dot(&hit.normal);
        let attenuation = DVec3::new(1.0, 1.0, 1.0);
        let eta = if cos_theta > 0.0 { 1.0 / self.eta } else { self.eta };
        let refraction = glm::refract_vec(&-ray.direction, &hit.normal.normalize(), eta);
        let ref_coeff = ((1.0-eta) / (1.0+eta)).powi(2);
        let ref_coeff_theta = (ref_coeff + (1.0 - ref_coeff) * (1.0 - cos_theta).powi(5)).clamp(0.0, 1.0);
        let mut rng = rand::thread_rng();

        let ray_dir = if refraction.norm_squared() < 1e-10 || rng.gen_bool(ref_coeff_theta) {
            glm::reflect_vec(&ray.direction, &hit.normal.normalize())
        } else {
            refraction
        };
        // let ray_dir = refraction;
        let ray = Ray::new(hit.point, ray_dir);
        vec![(attenuation, ray)]
    }

    fn emit(&self, _ray: &crate::hit::Ray, _hit: &crate::hit::HitRecord) -> nalgebra_glm::DVec3 {
        DVec3::zeros()
    }
}