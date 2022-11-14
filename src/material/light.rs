use nalgebra_glm::DVec3;
use serde::{Serialize, Deserialize};

use crate::hit::{Ray, HitRecord};
use nalgebra_glm as glm;
use super::Material;

#[derive(Clone, Serialize, Deserialize)]
pub struct Light {
    pub color_light: DVec3,
    pub radiance: f64
}

impl Light {
    pub fn new(color_light: DVec3, radiance: f64) -> Self {
        Self { color_light, radiance }
    }
}

#[typetag::serde]
impl Material for Light {
    fn scatter(&self, _: &crate::hit::Ray, _hit: &crate::hit::HitRecord) -> Vec<(DVec3, crate::hit::Ray)> {
        vec![]
    }
    fn emit(&self, _ray: &Ray, _hit: &HitRecord) -> glm::DVec3 {
        self.radiance * self.color_light
    }
}