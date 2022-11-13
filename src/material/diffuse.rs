use nalgebra_glm::DVec3;
use rand_distr::num_traits::Zero;

use crate::{hit::{Ray, HitRecord}, utils::random_in_unit_sphere};
use nalgebra_glm as glm;
use super::Material;

pub struct Diffuse {
    pub color_diffuse: DVec3,
}

impl Diffuse {
    pub fn new(color_diffuse: DVec3) -> Self {
        Self { color_diffuse }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _: &crate::hit::Ray, hit: &crate::hit::HitRecord) -> Vec<(DVec3, crate::hit::Ray)> {
        let mut direction = hit.normal + random_in_unit_sphere().normalize();
        if direction.norm_squared() < 1e-8 {
            direction = hit.normal;
        }
        let ray_scattered = Ray::new(hit.point, direction);
        vec![
            (self.color_diffuse, ray_scattered)
        ]
    }
    fn emit(&self, _ray: &Ray, _hit: &HitRecord) -> glm::DVec3 {
        glm::DVec3::zeros()
    }
}