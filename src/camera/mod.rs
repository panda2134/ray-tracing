mod screen;

use glm::DVec3;
use nalgebra_glm as glm;
pub use screen::Screen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub screen: screen::Screen,
    pub viewport_height: f64,
    pub origin: glm::DVec3,
    pub focal_len: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            screen: Screen::default(),
            viewport_height: 2.0,
            focal_len: 1.0,
            origin: glm::DVec3::zeros(),
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Camera {
    fn rotation_matrix(&self) -> glm::DMat4 {
        glm::rotation(-self.pitch, &DVec3::new(0.0, 1.0, 0.0)) * glm::rotation(self.yaw, &DVec3::new(1.0, 0.0, 0.0))
    }

    pub fn viewport_width(&self) -> f64 {
        self.viewport_height * self.screen.aspect_ratio()
    }

    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    pub fn horizontal_vec(&self) -> glm::DVec3 {
        (self.rotation_matrix() * glm::DVec4::new(self.viewport_width(), 0.0, 0.0, 1.0)).xyz()
    }

    pub fn vertical_vec(&self) -> glm::DVec3 {
        (self.rotation_matrix() * glm::DVec4::new(0.0, self.viewport_height(), 0.0, 1.0)).xyz()
    }

    /// Orientation vector of the camera. Its length is equal to the focal length.
    pub fn orient_vec(&self) -> glm::DVec3 {
        (self.rotation_matrix() * glm::DVec4::new(0.0, 0.0, -self.focal_len, 1.0)).xyz()
    }

    pub fn left_bottom_vec(&self) -> glm::DVec3 {
        self.origin - 0.5 * self.horizontal_vec() - 0.5 * self.vertical_vec() + self.orient_vec()
    }
}
