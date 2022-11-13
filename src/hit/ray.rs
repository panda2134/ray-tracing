use bvh;

use nalgebra_glm as glm;
use glm::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3
}

impl From<&bvh::ray::Ray> for Ray {
    fn from(ray: &bvh::ray::Ray) -> Self {
        Self { origin: DVec3::new(ray.origin.x as f64, ray.origin.y as f64, ray.origin.z as f64),
            direction: DVec3::new(ray.direction.x as f64, ray.direction.y as f64, ray.direction.z as f64) }
    }
}

impl From<bvh::ray::Ray> for Ray {
    fn from(ray: bvh::ray::Ray) -> Self {
        Self::from(&ray)
    }
}

impl Into<bvh::ray::Ray> for Ray {
    fn into(self) -> bvh::ray::Ray {
        (&self).into()
    }
}

impl Into<bvh::ray::Ray> for &Ray {
    fn into(self) -> bvh::ray::Ray {
        bvh::ray::Ray::new(
            bvh::Point3::new(self.origin.x as f32, self.origin.y as f32, self.origin.z as f32), 
            bvh::Vector3::new(self.direction.x as f32, self.direction.y as f32, self.direction.z as f32), 
        )
    }
}
