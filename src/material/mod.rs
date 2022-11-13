use nalgebra_glm as glm;

use crate::{hit::{Ray, HitRecord}};

mod diffuse;
mod light;
mod wood;
mod metal;

pub use diffuse::*;
pub use light::*;
pub use wood::*;
pub use metal::*;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Vec<(glm::DVec3, Ray)>;
    fn emit(&self, ray: &Ray, hit: &HitRecord) -> glm::DVec3;
}