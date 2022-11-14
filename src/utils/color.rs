use nalgebra_glm::DVec3;

/// Convert any 3d vector into RGB color
pub fn vec_to_color(vec: DVec3) -> DVec3 {
  let n = vec.normalize();
  let lerp = |x: f64| (x + 1.0) / 2.0;
  DVec3::new(lerp(n.x), lerp(n.y), lerp(n.z))
}

pub const WHITE: DVec3 = DVec3::new(1.0, 1.0, 1.0);
pub const RED: DVec3 = DVec3::new(1.0, 0.0, 0.0);
pub const GREEN: DVec3 = DVec3::new(0.0, 1.0, 0.0);
pub const BLUE: DVec3 = DVec3::new(0.0, 0.0, 1.0);
pub const YELLOW: DVec3 = DVec3::new(1.0, 1.0, 0.0);
pub const CYAN: DVec3 = DVec3::new(0.0, 1.0, 1.0);
pub const PURPLE: DVec3 = DVec3::new(1.0, 0.0, 1.0);
pub const BLACK: DVec3 = DVec3::new(0.0, 0.0, 0.0);
