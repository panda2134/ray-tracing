use nalgebra_glm::DVec3;
use rand::prelude::*;
pub fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    let n = rand_distr::StandardNormal {};
    let u = rand_distr::Uniform::new(0.0, 1.0f64);

    let vec = DVec3::new(n.sample(&mut rng), n.sample(&mut rng), n.sample(&mut rng)).normalize();
    let k = u.sample(&mut rng).cbrt();
    
    k * vec
}