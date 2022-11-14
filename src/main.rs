use std::{sync::Arc, collections::HashMap};

use glm::{DVec3};
use hit::{BroadPhase, BroadPhaseShape, BVHBroadPhase};
use itertools::Itertools;
use material::{Wood, Metal, Dielectric, Material};
use rand::prelude::*;
use kdam::{tqdm};

mod camera;
mod hit;
mod shape;
mod tracer;
mod utils;
mod material;

use nalgebra_glm as glm;
use rayon::prelude::*;
use shape::{Shape, Sphere, load_triangle, draw_cube};
use tracer::{TracingHelper};
use utils::{WHITE, cornell_box};

fn main() -> anyhow::Result<()> {
    let screen = camera::Screen::new(400, 300);
    let mut output = image::Rgb32FImage::new(screen.width, screen.height);
    let camera = camera::Camera {
        screen,
        viewport_height: 2.0,
        origin: glm::DVec3::new(0.0, 0.0, 0.0),
        focal_len: 1.0,
        pitch: 0.0,
        yaw: 0.0,
    };
    let mut world: Vec<Arc<dyn Shape>> = vec![];

    let wood: Arc<dyn Material> = Arc::new(
        Wood::new(DVec3::zeros(), DVec3::new(0.0, 0.0, 1.0), DVec3::new(0.0, 1.0, 0.0), material::WoodType::RedWood)
    );
    let metal: Arc<dyn Material> = Arc::new(Metal::new(0.8 * WHITE, 0.05));
    let glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.6));
    let bunny = load_triangle(
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bunny.obj")),
        &glm::scale(&glm::translation(&DVec3::new(0.0, -0.75, -0.8)), &DVec3::new(2.0, 2.0, 2.0)),
        wood
    )?;
    for x in bunny {
        world.push(Arc::new(x))
    }

    let cube_translation = glm::translation(&DVec3::new(0.5, -0.85, -1.4))
     * glm::rotation(-1.0, &DVec3::new(0.0, 1.0, 0.0))
     * glm::scaling(&DVec3::new(0.5, 0.3, 0.5));
    let cube = draw_cube(&cube_translation, &metal);
    
    for x in cube {
        world.push(x)
    }

    world.append(&mut cornell_box());
    world.push(Arc::new(Sphere {
        center: DVec3::new(-1.0, -0.75, -1.3),
        radius: 0.25,
        material: glass,
    }));

    let mut obj: Vec<BroadPhaseShape> = world
        .iter()
        .map(|s| BroadPhaseShape::new(s.clone()))
        .collect();
    
    let mut broad_phase: Box<dyn BroadPhase> = Box::new(BVHBroadPhase::default());
    broad_phase.build(&mut obj);
    let tracing_helper = TracingHelper::new(&obj, broad_phase, 3);
    let samples_per_pixel = 100;

    let mut image_map = HashMap::<(u32, u32), DVec3>::new();
    let res = tqdm!((0..screen.width).cartesian_product(0..screen.height).cartesian_product(0..samples_per_pixel))
        .par_bridge()
        .into_par_iter()
        .map(|((x, y), _)| -> (u32, u32, glm::DVec3) {
            let mut rng = rand::thread_rng();
            let dist = rand::distributions::Uniform::new(0.0, 1.0f64);
            let u = (x as f64 + dist.sample(&mut rng)) / (screen.width as f64);
            let v = (y as f64 + dist.sample(&mut rng)) / (screen.height as f64);
            let ray_dir = camera.left_bottom_vec()
                + u * camera.horizontal_vec()
                + v * camera.vertical_vec();

            let ray = hit::Ray {
                origin: camera.origin,
                direction: ray_dir,
            };
            let color = tracing_helper.start_trace(&ray);
            (x, y, color)
        })
        .collect::<Vec<(u32, u32, DVec3)>>();
    
    for (i, j, color) in res {
        let k = 1.0 / (samples_per_pixel as f64);
        
        *image_map.entry((i, j)).or_insert(DVec3::zeros()) += k * color;
    }
    for (i, j) in (0..screen.width).cartesian_product(0..screen.height) {
        let vec = image_map.get(&(i, j)).unwrap();
        let vec_f32 = [vec.x as f32, vec.y as f32, vec.z as f32];
        // Coordinate system differs: +y on the screen becomes -y in the picture
        output.put_pixel(i, screen.height - 1 - j, image::Rgb::<f32>(vec_f32.into()))
    }
    let conv = image::DynamicImage::ImageRgb32F(output).into_rgb8();
    conv.save("output.png")?;
    Ok(())
}
