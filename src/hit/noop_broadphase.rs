use bvh::bounding_hierarchy::BHShape;
use itertools::Itertools;

use super::{BroadPhase, BroadPhaseShape, ray};

#[derive(Default)]
pub struct NoOpBroadPhase;

impl BroadPhase for NoOpBroadPhase {
    fn trace<'a>(&'a self, shapes: &'a [BroadPhaseShape], _ray: &ray::Ray) -> Vec<&'a BroadPhaseShape> {
      shapes.iter().map(|x| x).collect_vec()
    }

    fn build(&mut self, shapes: &mut [BroadPhaseShape]) {
      for (i, s) in shapes.iter_mut().enumerate() {
        s.set_bh_node_index(i)
      }
    }
}