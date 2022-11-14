mod sphere;
mod triangle;

use std::sync::Arc;

use crate::{hit::{HitRecord, Ray}, material::Material};
use bvh::aabb::Bounded;

pub use sphere::Sphere;
pub use triangle::*;

#[typetag::serde(tag = "type")]
pub trait Shape: Bounded + Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;

    /// returns Some() if the toi lies in the given bound (min_val, max_val);
    /// returns None otherwise.
    fn hit_with_bound(&self, ray: &Ray, bound: (f64, f64)) -> Option<HitRecord> {
        assert!(bound.0 <= bound.1);
        self.hit(ray).and_then(|h|
            if h.toi >= bound.0 && h.toi <= bound.1 { Some(h) }
            else { None }
        )
    }

    fn material(&self, hit: &HitRecord) -> Arc<dyn Material>;
}