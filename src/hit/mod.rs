mod bvh_broadphase;
mod noop_broadphase;
mod broadphase_shape;
mod ray;





use nalgebra_glm as glm;

pub use bvh_broadphase::BVHBroadPhase;
pub use noop_broadphase::NoOpBroadPhase;
pub use ray::Ray;
pub use broadphase_shape::BroadPhaseShape;

pub struct HitRecord {
    pub toi: f64,
    pub point: glm::TVec3<f64>,
    pub normal: glm::TVec3<f64>,
}

pub trait BroadPhase: Default {
    /// return shapes that can *possibly* intersect with the ray.
    fn trace<'a>(&'a self, shapes: &'a [BroadPhaseShape], ray: &ray::Ray) -> Vec<&'a BroadPhaseShape>;

    /// build the underlying data structure of broad phase
    fn build(&mut self, shapes: &mut [BroadPhaseShape]);
}