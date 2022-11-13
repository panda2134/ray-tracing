use std::sync::Arc;

use glm::DVec3;
use hit::{BroadPhase, BroadPhaseShape, HitRecord};
use itertools::Itertools;
use material::{Diffuse, Light, Wood};
use rand::prelude::*;

mod camera;
mod hit;
mod shape;
mod tracer;
mod utils;
mod material;

use nalgebra_glm as glm;
use rayon::prelude::*;
use shape::{Shape, Sphere};
use tracer::{TracingHelper};
use utils::{BLACK, BLUE, CYAN, GREEN, RED, WHITE};

fn main() -> anyhow::Result<()> {
    let screen = camera::Screen::default();
    let mut output = image::Rgb32FImage::new(screen.width, screen.height);
    let camera = camera::Camera {
        screen,
        viewport_height: 2.0,
        origin: glm::zero(),
        focal_len: 1.0,
        pitch: 0.0,
        yaw: 0.0,
    };
    let mut broad_phase = hit::BVHBroadPhase::default();
    let world: Vec<Arc<dyn Shape>> = vec![
        Arc::new(Sphere {
            center: glm::DVec3::new(0.0, -1000.5, -2.0),
            radius: 1000.0,
            material: Arc::new(Wood::new(
                DVec3::new(-1000.0, 0.0, -1000.0),
                DVec3::new(0.0, 0.0, 1000.0),
                DVec3::new(1000.0, 0.0, 0.0),
            ))
        }),
        Arc::new(Sphere {
            center: glm::DVec3::new(0.0, 0.0, -2.0),
            radius: 0.5,
            material: Arc::new(Diffuse::new(DVec3::new(0.25, 0.35, 0.5)))
        }),
        Arc::new(Sphere {
            center: glm::DVec3::new(0.0, 100.0 + 40.0, 0.0),
            radius: 100.0,
            material: Arc::new(
                Light::new(WHITE, 1.0)
            )
        }),
    ];
    let mut obj: Vec<BroadPhaseShape> = world
        .iter()
        .map(|s| BroadPhaseShape::new(s.clone()))
        .collect();
    broad_phase.build(&mut obj);
    let tracing_helper = TracingHelper::new(&obj, &broad_phase, 4);
    let samples_per_pixel = 100;

    let res: Vec<(u32, u32, glm::DVec3)> = (0..screen.width)
        .cartesian_product(0..screen.height)
        .par_bridge()
        .into_par_iter()
        .map(|(x, y)| -> (u32, u32, glm::DVec3) {
            let mut rng = rand::thread_rng();
            let dist = rand::distributions::Uniform::new(0.0, 1.0f64);
            let mut color = DVec3::default();
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + dist.sample(&mut rng)) / (screen.width as f64);
                let v = (y as f64 + dist.sample(&mut rng)) / (screen.height as f64);
                let ray_dir = camera.left_bottom_vec()
                    + u * camera.horizontal_vec()
                    + v * camera.vertical_vec();

                let ray = hit::Ray {
                    origin: camera.origin,
                    direction: ray_dir,
                };
                color += tracing_helper.start_trace(&ray);
            }
            color /= samples_per_pixel as f64;
            (x, y, color)
        })
        .collect();
    for (i, j, color) in res {
        let vec_f32 = glm::vec3(color.x as f32, color.y as f32, color.z as f32);
        // Coordinate system differs: +y on the screen becomes -y in the picture
        output.put_pixel(i, screen.height - 1 - j, image::Rgb::<f32>(vec_f32.into()))
    }
    let conv = image::DynamicImage::ImageRgb32F(output).into_rgb8();
    conv.save("output.png")?;
    Ok(())
}
