use std::sync::Arc;

use cached::proc_macro::cached;
use nalgebra_glm::DVec3;
use serde::{Serialize, Deserialize};

use crate::{hit::{Ray, HitRecord}, utils::random_in_unit_sphere};
use nalgebra_glm as glm;
use super::Material;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum WoodType {
    Wood,
    RedWood
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Wood {
    pub origin: DVec3,
    pub h: DVec3,
    pub w: DVec3,
    pub wood_type: WoodType
}

#[cached]
fn wood_texture(wood_type: WoodType) -> Arc<image::Rgb32FImage> {
    match wood_type {
        WoodType::Wood => {
            let texture_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wood.jpg"));
            let texture = image::load_from_memory(texture_bytes).unwrap();
            Arc::new(texture.into_rgb32f())
        }
        WoodType::RedWood => {
            let texture_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/red_wood.jpg"));
            let texture = image::load_from_memory(texture_bytes).unwrap();
            Arc::new(texture.into_rgb32f())
        }
    }
}

impl Wood {
    pub fn new(origin: DVec3, h: DVec3, w: DVec3, wood_type: WoodType) -> Self {
        Self {
            origin,
            wood_type,
            h: h.normalize(),
            w: w.normalize(),
        }
    }

    fn get_color_at(&self, point: DVec3) -> DVec3 {
        let vp = point - self.origin;
        let u = vp.dot(&self.w) - vp.dot(&self.w).floor();
        let v = vp.dot(&self.h) - vp.dot(&self.h).floor();

        let texture = wood_texture(self.wood_type);

        let iw = u * (texture.width() as f64);
        let ih = v * (texture.height() as f64);

        let pixel = texture.get_pixel(iw as u32, ih as u32).0;
        DVec3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}

#[typetag::serde]
impl Material for Wood {
    fn scatter(&self, _: &crate::hit::Ray, hit: &crate::hit::HitRecord) -> Vec<(DVec3, crate::hit::Ray)> {
        let ray_scattered = Ray::new(hit.point, hit.normal + random_in_unit_sphere().normalize());

        vec![
            (self.get_color_at(hit.point), ray_scattered)
        ]
    }
    fn emit(&self, _ray: &Ray, _hit: &HitRecord) -> glm::DVec3 {
        glm::DVec3::zeros()
    }
}