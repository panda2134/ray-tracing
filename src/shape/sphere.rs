use bvh::{aabb::Bounded};
use nalgebra_glm as glm;
use crate::hit::HitRecord;
use super::Shape;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: glm::DVec3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: glm::DVec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> bvh::aabb::AABB {
        let quad_cube = glm::vec3(self.radius, self.radius, self.radius);
        let min = self.center - quad_cube;
        let max = self.center + quad_cube;
        bvh::aabb::AABB {
            min: bvh::Vector3::new(min.x as f32, min.y as f32, min.z as f32),
            max: bvh::Vector3::new(max.x as f32, max.y as f32, max.z as f32),
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &crate::hit::Ray) -> Option<crate::hit::HitRecord> {
        /*
         1. p = o + k * ray_dir
         2. |p-c| = r
         => |ray_dir|^2 k^2 + 2 * ray_dir * (o-c) * k + |o-c|^2 - r^2 = 0
        */
        let origin = ray.origin;
        let ray_dir = ray.direction;

        let oc = origin - self.center;

        let a = ray_dir.norm_squared();
        let b = 2.0 * ray_dir.dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            None
        } else {
            let sqrt_delta = delta.sqrt();
            let t1 = (-b - sqrt_delta) / (2.0 * a);
            let t2 = (-b + sqrt_delta) / (2.0 * a);
            if t1 < 0.0 && t2 < 0.0 {
                None
            } else {
                let toi = if a > 0.0 {
                    if t1 >= 0.0 { t1 } else { t2 } 
                } else {
                    if t2 >= 0.0 { t2 } else { t1 }
                };
                let point = origin + toi * ray_dir;
                Some(HitRecord {
                    toi,
                    point,
                    normal: (point - self.center).normalize(),
                })
            }
        }
    }
}
