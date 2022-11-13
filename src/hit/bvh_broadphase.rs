use super::{BroadPhase, broadphase_shape::BroadPhaseShape, ray};
use bvh::bvh::BVH;

#[derive(Default)]
pub struct BVHBroadPhase {
  bvh: Option<BVH>
}

impl BroadPhase for BVHBroadPhase {
    fn trace<'a>(&'a self, shapes: &'a [BroadPhaseShape], ray: &ray::Ray) -> Vec<&'a BroadPhaseShape> {
        self.bvh.as_ref().expect("BVHBroadPhase not initialized").traverse(&ray.into(), shapes).into_iter().map(|x| x.to_owned()).collect()
    }

    fn build(&mut self, shapes: &mut [BroadPhaseShape]) {
        self.bvh = Some(BVH::build(shapes));
    }
}