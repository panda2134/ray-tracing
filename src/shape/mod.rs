mod sphere;

use crate::hit::{HitRecord, Ray};
use bvh::aabb::Bounded;

pub use sphere::Sphere;

pub trait Shape: Bounded + Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}