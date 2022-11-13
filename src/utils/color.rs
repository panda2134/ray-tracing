use nalgebra_glm::DVec3;

/// Convert any 3d vector into RGB color
pub fn vec_to_color(vec: DVec3) -> DVec3 {
  let n = vec.normalize();
  let lerp = |x: f64| (x + 1.0) / 2.0;
  DVec3::new(lerp(n.x), lerp(n.y), lerp(n.z))
}