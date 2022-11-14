use nalgebra_glm as glm;
use serde::{Serialize, Deserialize};

use crate::hit::{Ray, HitRecord};
use typetag;

mod diffuse;
mod light;
mod wood;
mod metal;
mod dielectric;

pub use diffuse::*;
pub use light::*;
pub use wood::*;
pub use metal::*;
pub use dielectric::*;

#[typetag::serde(tag = "type")]
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Vec<(glm::DVec3, Ray)>;
    fn emit(&self, ray: &Ray, hit: &HitRecord) -> glm::DVec3;
}