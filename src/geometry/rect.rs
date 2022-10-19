use crate::geometry::Line;
use crate::vector::Vector2;

#[derive(Copy, Clone)]
pub struct Rect {
    /// co-ordinates of a corner of the rectangle
    pub origin: Vector2,
    /// relative to origin
    pub size: Vector2,
}

impl Rect {
    pub fn new(origin: Vector2, width: f32, height: f32) -> Self {
        Self {
            origin,
            size: Vector2::new(width, height),
        }
    }

    pub fn square(origin: Vector2, size: f32) -> Self {
        Self::new(origin, size, size)
    }

    pub fn x(&self) -> Line {
        Line::new(self.origin.x, self.size.x)
    }

    pub fn y(&self) -> Line {
        Line::new(self.origin.y, self.size.y)
    }

    pub fn contains(&self, point: Vector2) -> bool {
        self.x().contains(point.x) && self.y().contains(point.y)
    }

    pub fn intersects(&self, other: Rect) -> bool {
        self.x().intersects(other.x()) && self.y().intersects(other.y())
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::rect::Rect;
    use crate::vector::Vector2;

    #[test]
    fn contains_point() {
        let square = Rect::square(Vector2::new(0.0, 0.0), 5.0);
        assert!(square.contains(Vector2::new(0.0, 0.0)));
        assert!(square.contains(Vector2::new(5.0, 5.0)));
        assert!(square.contains(Vector2::new(2.5, 2.5)));
        assert!(!square.contains(Vector2::new(5.0, 6.0)));
        assert!(!square.contains(Vector2::new(6.0, 5.0)));
        assert!(!square.contains(Vector2::new(-1.0, 2.5)));
    }

    #[test]
    fn intersects() {
        let square = Rect::square(Vector2::new(0.0, 0.0), 5.0);
        let other = Rect::square(Vector2::new(2.0, 3.0), 5.0);
        assert!(square.intersects(other));
        let other = Rect::square(Vector2::new(10.0, 10.0), 5.0);
        assert!(!square.intersects(other));
    }
}
