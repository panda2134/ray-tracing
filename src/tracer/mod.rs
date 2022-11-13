use itertools::Itertools;
use nalgebra_glm::DVec3;

use crate::{
    hit::{BroadPhase, BroadPhaseShape, HitRecord, Ray},
    utils,
};
use nalgebra_glm as glm;

const black: DVec3 = DVec3::new(0.0, 0.0, 0.0);

#[derive(Copy, Clone)]
pub struct Light {
  pub position: DVec3,
  pub color: DVec3
}

pub struct TracingHelper<'a, T: BroadPhase> {
    obj: &'a Vec<BroadPhaseShape>,
    broad_phase: &'a T,
    pub light: Light,
    depth_limit: usize,
}

impl<'a, T: BroadPhase> TracingHelper<'a, T> {
    pub fn new(
        obj: &'a Vec<BroadPhaseShape>,
        broad_phase: &'a T,
        light: Light,
        depth_limit: usize,
    ) -> TracingHelper<'a, T> {
        TracingHelper {
            obj,
            broad_phase,
            light,
            depth_limit,
        }
    }

    /// start ray tracing. stop after given depth
    pub fn start_trace(&self, ray: &Ray) -> DVec3 {
        self.trace(ray, self.depth_limit)
    }

    fn trace(&self, ray: &Ray, depth: usize) -> DVec3 {
        if depth == 0 {
            return black; // bloack
        }
        let records = self.ray_intersect(ray);
        let nearest_hit = records.first();

        match nearest_hit {
            Some(hit_info) => {
                let (hit, bf_shape) = hit_info;
                let mut light_strength = DVec3::new(0.0, 0.0, 0.0);

                let ray_dir_from_light = hit.point - self.light.position;
                let ray_light = Ray { origin: self.light.position, direction: ray_dir_from_light };
                let light_available = self.ray_intersect_with_bound(&ray_light, (0.0, 1.0 - 1e-6)).is_empty();
                if light_available {
                    light_strength += 0.8 * self.light.color.component_mul(&bf_shape.shape.color(&hit));
                }

                let n = hit.normal.normalize();

                let reflect_dir = glm::reflect_vec(&ray.direction, &n);
                let reflect_ray = Ray { origin: hit.point, direction: reflect_dir };
                let reflect_light = self.trace(&reflect_ray, depth - 1);
                light_strength += bf_shape.shape.reflection_attenuation(&hit) * reflect_light;

                let refract_dir = glm::refract_vec(&ray.direction, &n, bf_shape.shape.refraction_index(&hit_info.0));
                let refract_ray = Ray { origin: hit.point, direction: refract_dir };
                let refract_light = self.trace(&refract_ray, depth - 1);
                light_strength += bf_shape.shape.refraction_attenuation(&hit) * refract_light;

                light_strength
            }
            None => black,
        }
    }

    fn ray_intersect(&self, ray: &Ray) -> Vec<(HitRecord, &BroadPhaseShape)> {
        let filtered = self.broad_phase.trace(&self.obj, ray);
        let records = filtered
            .into_iter()
            .flat_map(|x| x.shape.hit(ray).and_then(|h| Some((h, x))))
            .sorted_by(|x, y| x.0.toi.partial_cmp(&y.0.toi).unwrap())
            .collect_vec();
        records
    }

    fn ray_intersect_with_bound(&self, ray: &Ray, bound: (f64, f64)) -> Vec<(HitRecord, &BroadPhaseShape)> {
      let filtered = self.broad_phase.trace(&self.obj, ray);
      let records = filtered
          .into_iter()
          .flat_map(|x| x.shape.hit_with_bound(ray, bound).and_then(|h| Some((h, x))))
          .sorted_by(|x, y| x.0.toi.partial_cmp(&y.0.toi).unwrap())
          .collect_vec();
      records
  }
}
