use std::sync::Arc;

use nalgebra_glm::DVec3;

use crate::{
    material::{Diffuse, Light, Material, Wood},
    shape::{draw_rect, Shape},
};

use super::color::*;

pub fn cornell_box() -> Vec<Arc<dyn Shape>> {
    let mut world: Vec<Arc<dyn Shape>> = vec![];
    let green: Arc<dyn Material> = Arc::new(Diffuse::new(GREEN));
    let red: Arc<dyn Material> = Arc::new(Diffuse::new(RED));
    let white: Arc<dyn Material> = Arc::new(Diffuse::new(WHITE));
    let wood: Arc<dyn Material> = Arc::new(Wood::new(
        DVec3::new(-2.0, -1.0, -2.0),
        DVec3::new(1.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 1.0),
        crate::material::WoodType::Wood
    ));

    world.extend(
        draw_rect(
            &[
                DVec3::new(-2.0, -1.0, 1.0),
                DVec3::new(-2.0, -1.0, -2.0),
                DVec3::new(-2.0, 1.0, -2.0),
                DVec3::new(-2.0, 1.0, 1.0),
            ],
            &green,
        )
        .into_iter(),
    );

    world.extend(
        draw_rect(
            &[
                DVec3::new(2.0, -1.0, 1.0),
                DVec3::new(2.0, 1.0, 1.0),
                DVec3::new(2.0, 1.0, -2.0),
                DVec3::new(2.0, -1.0, -2.0),
            ],
            &red,
        )
        .into_iter(),
    );

    world.extend(
        draw_rect(
            &[
                DVec3::new(-2.0, -1.0, -2.0),
                DVec3::new(-2.0, -1.0, 1.0),
                DVec3::new(2.0, -1.0, 1.0),
                DVec3::new(2.0, -1.0, -2.0),
            ],
            &wood,
        )
        .into_iter(),
    );

    world.extend(
        draw_rect(
            &[
                DVec3::new(-2.0, 1.0, -2.0),
                DVec3::new(-2.0, -1.0, -2.0),
                DVec3::new(2.0, -1.0, -2.0),
                DVec3::new(2.0, 1.0, -2.0),
            ],
            &white,
        )
        .into_iter(),
    );

    world.extend(
        draw_rect(
            &[
                DVec3::new(-2.0, 1.0, 1.0),
                DVec3::new(2.0, 1.0, 1.0),
                DVec3::new(2.0, -1.0, 1.0),
                DVec3::new(-2.0, -1.0, 1.0),
            ],
            &white,
        )
        .into_iter(),
    );

    world.extend(
        draw_rect(
            &[
                DVec3::new(-1.0, 1.0, -1.8),
                DVec3::new(1.0, 1.0, -1.8),
                DVec3::new(1.0, 1.0, 0.8),
                DVec3::new(-1.0, 1.0, 0.8),
            ],
            &(Arc::new(Light::new(DVec3::new(1.0, 1.0, 0.8), 5.0)) as Arc<dyn Material>),
        )
        .into_iter(),
    );

    world.extend(
        draw_rect(
            &[
                DVec3::new(-2.0, 1.0, -2.0),
                DVec3::new(2.0, 1.0, -2.0),
                DVec3::new(2.0, 1.0, 1.0),
                DVec3::new(-2.0, 1.0, 1.0),
            ],
            &white,
        )
        .into_iter(),
    );
    world
}
