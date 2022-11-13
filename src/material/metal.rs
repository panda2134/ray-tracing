use nalgebra_glm::DVec3;

use crate::{hit::Ray, utils};

use super::Material;
use nalgebra_glm as glm;

pub struct Metal {
    pub color: DVec3,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(color: DVec3, fuzziness: f64) -> Self {
        Self { color, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::hit::Ray,
        hit: &crate::hit::HitRecord,
    ) -> Vec<(nalgebra_glm::DVec3, crate::hit::Ray)> {
        if ray.direction.dot(&hit.normal) > 0.0 {
            vec![]
        } else {
            let reflected = glm::reflect_vec(&ray.direction, &hit.normal).normalize()
                + self.fuzziness * utils::random_in_unit_sphere();
            let ray_new = Ray::new(hit.point, reflected);
            vec![(self.color, ray_new)]
        }
    }

    fn emit(&self, _ray: &crate::hit::Ray, _hit: &crate::hit::HitRecord) -> nalgebra_glm::DVec3 {
        DVec3::zeros()
    }
}
