mod sphere;

use crate::hit::{HitRecord, Ray};
use bvh::aabb::Bounded;

use nalgebra_glm::DVec3;
pub use sphere::Sphere;

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

    fn color(&self, hit: &HitRecord) -> DVec3;
    fn reflection_attenuation(&self, hit: &HitRecord) -> f64;
    fn refraction_attenuation(&self, hit: &HitRecord) -> f64;
    fn refraction_index(&self, hit: &HitRecord) -> f64;
}