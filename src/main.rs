use std::sync::Arc;

use glm::DVec3;
use hit::{BroadPhase, BroadPhaseShape, HitRecord};
use itertools::Itertools;
use rand::prelude::*;

mod camera;
mod hit;
mod shape;
mod utils;
mod trace;

use nalgebra_glm as glm;
use rayon::prelude::*;
use shape::{Sphere, Shape};

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
        Arc::new(Sphere::new(glm::DVec3::new(0.0, 0.0, -1.0), 0.5)),
        Arc::new(Sphere::new(glm::DVec3::new(0.0, -100.5, -1.0), 100.0))
    ];
    let mut obj: Vec<BroadPhaseShape> = world.iter().map(|s| BroadPhaseShape::new(s.clone())).collect();
    broad_phase.build(&mut obj);
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
                let u = (x as f64 + dist.sample(&mut rng)) / (screen.width as f64) ;
                let v = (y as f64 + dist.sample(&mut rng)) / (screen.height as f64) ;
                let ray_dir =
                    camera.left_bottom_vec() + u * camera.horizontal_vec() + v * camera.vertical_vec();

                let ray = hit::Ray { origin: camera.origin, direction: ray_dir };
                let filtered = broad_phase.trace(&obj, &ray);
                
                let t = 0.5 * (ray_dir.normalize().y + 1.0);
                let bg =
                    (1.0 - t) * glm::DVec3::new(1.0, 1.0, 1.0) + t * glm::DVec3::new(0.5, 0.7, 1.0);
                let mut records: Vec<HitRecord> = vec![];
                for x in filtered {
                    if let Some(h) = x.shape.hit(&ray) {
                        records.push(h);
                    }
                }
                records.sort_by(|x, y| x.toi.partial_cmp(&y.toi).unwrap());
                color += records.first().map_or(bg, |x| utils::vec_to_color(x.normal));
            }
            (x, y, color / (samples_per_pixel as f64))
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