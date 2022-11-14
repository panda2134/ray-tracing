use std::{sync::Arc, collections::HashMap, fs::File, io::BufReader};

use glm::{DVec3};
use hit::{BroadPhase, BroadPhaseShape, BVHBroadPhase, NoOpBroadPhase};
use itertools::Itertools;
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
use shape::{Shape};
use tracer::{TracingHelper};
use utils::{cornell_box, SceneInfo};
use clap::{Parser, arg, command};

/// A Simple PBR ray tracer
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Scene file. If not given, the default scene will be used
    #[arg(short = 'i', long)]
    scene_file: Option<String>,

    /// Dump the default scene to stdout and exit
    #[arg(short = 'd', long)]
    dump_scene: bool,

    /// How many layers it will trace before stopping
    #[arg(short = 'l', long, default_value_t = 3)]
    depth_limit: usize,

    /// Samples per pixel
    #[arg(short = 's', long, default_value_t = 100)]
    samples_per_pixel: i32,

    /// Skip BVH broad phase for ray intersection test. This is very slow.
    #[arg(short = 'b', long, default_value_t = false)]
    skip_bvh: bool,

    /// Output file.
    #[arg(short = 'o', long, default_value = "output.png")]
    output_file: String
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut scene = SceneInfo::default();
    if args.dump_scene {
        println!("{}", serde_json::to_string_pretty(&scene).unwrap());
        return Ok(())
    }
    if let Some(ref path) = args.scene_file {
        let f = File::open(path)?;
        scene = serde_json::from_reader(BufReader::new(f))?;
    }

    let screen = &scene.camera.screen;
    let camera = &scene.camera;
    let mut output = image::Rgb32FImage::new(screen.width, screen.height);
    let mut world: Vec<Arc<dyn Shape>> = cornell_box();

    world.append(&mut scene.split_to_shape());

    let mut obj: Vec<BroadPhaseShape> = world
        .iter()
        .map(|s| BroadPhaseShape::new(s.clone()))
        .collect();
    
    let mut broad_phase: Box<dyn BroadPhase> = if args.skip_bvh {
        Box::new(NoOpBroadPhase::default())
    } else {
        Box::new(BVHBroadPhase::default())
    };
    broad_phase.build(&mut obj);
    let tracing_helper = TracingHelper::new(&obj, broad_phase, args.depth_limit);

    let mut image_map = HashMap::<(u32, u32), DVec3>::new();
    let res = tqdm!((0..screen.width).cartesian_product(0..screen.height).cartesian_product(0..args.samples_per_pixel))
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
        let k = 1.0 / (args.samples_per_pixel as f64);
        
        *image_map.entry((i, j)).or_insert(DVec3::zeros()) += k * color;
    }
    for (i, j) in (0..screen.width).cartesian_product(0..screen.height) {
        let vec = image_map.get(&(i, j)).unwrap();
        let vec_f32 = [vec.x as f32, vec.y as f32, vec.z as f32];
        // Coordinate system differs: +y on the screen becomes -y in the picture
        output.put_pixel(i, screen.height - 1 - j, image::Rgb::<f32>(vec_f32.into()))
    }
    let conv = image::DynamicImage::ImageRgb32F(output).into_rgb8();
    conv.save(args.output_file)?;
    Ok(())
}
