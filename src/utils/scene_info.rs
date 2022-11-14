use std::sync::Arc;


use nalgebra_glm::{DMat4, DVec3};
use nalgebra_glm as glm;
use serde::{Deserialize, Serialize};

use crate::camera::{Screen, Camera};
use crate::material::{Metal, Dielectric, self, Light};
use crate::utils::WHITE;
use crate::{shape::{Sphere, Shape, load_triangle, draw_cube}, material::{Material, Wood}};

use super::GREEN;

#[derive(Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub transform: DMat4,
    pub material: Arc<dyn Material>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SceneInfo {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub cubes: Vec<ModelInfo>,
    pub bunnies: Vec<ModelInfo>
}

impl Default for SceneInfo {
    fn default() -> SceneInfo {
        let screen = Screen::new(400, 300);
        let camera = Camera {
            screen,
            viewport_height: 2.0 * 3.0f64.sqrt(),
            origin: glm::DVec3::new(0.0, 0.0, 0.0),
            focal_len: 1.0,
            pitch: 0.0,
            yaw: 0.0,
        };
        let wood: Arc<dyn Material> = Arc::new(
            Wood::new(DVec3::new(0.0, -0.75, -0.5), DVec3::new(0.0, 1.0, 1.0), DVec3::new(0.0, 0.0, 1.0), material::WoodType::RedWood)
        );
        let metal: Arc<dyn Material> = Arc::new(Metal::new(0.8 * WHITE, 0.05));
        let glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.6));
        let bunny_transform = glm::scale(&glm::translation(&DVec3::new(0.0, -0.75, -0.5)), &DVec3::new(2.0, 2.0, 2.0));
        let cube_transform = glm::translation(&DVec3::new(1.0, -0.25, -1.4))
        * glm::rotation(-1.0, &DVec3::new(0.0, 1.5, 0.0))
        * glm::scaling(&DVec3::new(0.5, 1.0, 0.5));
        let glass_ball = Sphere {
            center: DVec3::new(0.0, -0.75, -1.3),
            radius: 0.25,
            material: glass,
        };
        let metal_ball = Sphere {
            center: DVec3::new(-1.0, -0.75, -1.0),
            radius: 0.25,
            material: metal.clone()
        };
        let light_ball = Sphere {
            center: DVec3::new(1.5, -0.975, -1.3),
            radius: 0.05,
            material: Arc::new(Light::new(GREEN, 100.0))
        };
        let scene = SceneInfo {
            camera,
            bunnies: vec![ModelInfo { transform: bunny_transform, material: wood }],
            cubes: vec![ModelInfo { transform: cube_transform, material: metal }],
            spheres: vec![glass_ball, metal_ball, light_ball]
        };
        scene
    }
}

impl SceneInfo {
    pub fn split_to_shape(&self) -> Vec<Arc<dyn Shape>> {
        let bunny_model = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bunny.obj"));

        let bunny_shapes = self.bunnies.iter().flat_map(|b| load_triangle(bunny_model, &b.transform, b.material.clone()).unwrap())
                                        .map(|b| Arc::new(b) as Arc<dyn Shape>);
        let cube_shapes = self.cubes.iter().flat_map(|c| draw_cube(&c.transform, &c.material));
        let sphere_shapes = self.spheres.iter().map(|s| Arc::new(s.clone()) as Arc<dyn Shape>);

        bunny_shapes.chain(cube_shapes).chain(sphere_shapes).collect()
    }
}