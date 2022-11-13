use nalgebra_glm::DVec3;

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
        let ray_scattered = Ray::new(hit.point, hit.normal + random_in_unit_sphere().normalize());
        vec![
            (self.color_diffuse, ray_scattered)
        ]
    }
    fn emit(&self, _ray: &Ray, _hit: &HitRecord) -> glm::DVec3 {
        glm::DVec3::zeros()
    }
}