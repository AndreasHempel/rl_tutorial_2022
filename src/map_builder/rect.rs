#[derive(Debug, Clone)]
pub struct Rect {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
}

impl Rect {
    /// TODO: Enforce that width and height are positive or swap the corner points around accordingly
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    /// Return true if self intersects with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (u32, u32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn width(&self) -> u32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> u32 {
        self.y2 - self.y1
    }
}
