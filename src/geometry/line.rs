#[derive(Copy, Clone)]
pub struct Line {
    origin: f32,
    size: f32,
}

impl Line {
    pub fn new(origin: f32, size: f32) -> Self {
        Self { origin, size }
    }

    pub fn contains(self, point: f32) -> bool {
        let (a, b) = (self.origin, self.origin + self.size);
        (a.min(b)..=a.max(b)).contains(&point)
    }

    pub fn intersects(self, other: Line) -> bool {
        self.contains(other.origin) || self.contains(other.origin + other.size)
    }
}
