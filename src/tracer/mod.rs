use itertools::Itertools;
use nalgebra_glm::DVec3;

use crate::hit::{BroadPhase, BroadPhaseShape, HitRecord, Ray};
use crate::utils::*;

use nalgebra_glm as glm;

pub struct TracingHelper<'a, T: BroadPhase> {
    obj: &'a Vec<BroadPhaseShape>,
    broad_phase: &'a T,
    depth_limit: usize,
}

impl<'a, T: BroadPhase> TracingHelper<'a, T> {
    pub fn new(
        obj: &'a Vec<BroadPhaseShape>,
        broad_phase: &'a T,
        depth_limit: usize,
    ) -> TracingHelper<'a, T> {
        TracingHelper {
            obj,
            broad_phase,
            depth_limit,
        }
    }

    /// start ray tracing. stop after given depth
    pub fn start_trace(&self, ray: &Ray) -> DVec3 {
        self.trace(ray, self.depth_limit)
    }

    fn trace(&self, ray: &Ray, depth: usize) -> DVec3 {
        if depth == 0 {
            return BLACK; // bloack
        }
        let records = self.ray_intersect(ray);
        let nearest_hit = records.first();

        match nearest_hit {
            Some(hit_info) => {
                let (hit, bf_shape) = hit_info;
                let material = bf_shape.shape.material(&hit);

                let rays_scattered = material.scatter(&ray, &hit);

                let res = rays_scattered.into_iter().map(|(c, r)| 
                    c.component_mul(&self.trace(&r, depth - 1))
                ).sum::<DVec3>() + material.emit(&ray, &hit);

                res
            }
            None => BLACK,
            // None => {
            //     let mut t = 0.5 * (ray.direction.normalize().y + 1.0);
            //     if ! t.is_finite() {t = 0.0;}
            //     glm::lerp(&WHITE, &DVec3::new(0.5, 0.7, 1.0), t)
            // },
        }
    }

    fn ray_intersect(&self, ray: &Ray) -> Vec<(HitRecord, &BroadPhaseShape)> {
        self.ray_intersect_with_bound(ray, (1e-8, 1e8))
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
