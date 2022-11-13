#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

impl Screen {
    pub fn aspect_ratio(&self) -> f64 {
        (self.width as f64) / (self.height as f64)
    }
}
