use bvh::bounding_hierarchy::BHShape;

use bvh;

use bvh::aabb::Bounded;

use crate::shape::Shape;

use std::sync::Arc;


pub struct BroadPhaseShape {
    pub shape: Arc<dyn Shape>,
    pub(crate) node_index: usize
}

impl Bounded for BroadPhaseShape {
    fn aabb(&self) -> bvh::aabb::AABB {
        self.shape.aabb()
    }
}

impl BHShape for BroadPhaseShape {
    fn set_bh_node_index(&mut self, i: usize) {
        self.node_index = i
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl BroadPhaseShape {
    pub fn new(shape: Arc<dyn Shape>) -> Self {
        Self {
            shape, node_index: Default::default()
        }
    }
}
