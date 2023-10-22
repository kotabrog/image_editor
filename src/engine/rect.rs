#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn to_center(&mut self, width: f64, height: f64) {
        self.x = (width - self.width) / 2.0;
        self.y = (height - self.height) / 2.0;
    }
}
