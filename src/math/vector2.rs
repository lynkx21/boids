#[allow(dead_code)]
#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    pub fn one() -> Vector2 {
        Vector2::new(1.0, 1.0)
    }

    pub fn right() -> Vector2 {
        Vector2::new(1.0, 0.0)
    }

    pub fn left() -> Vector2 {
        Vector2::new(-1.0, 0.0)
    }

    pub fn up() -> Vector2 {
        Vector2::new(0.0, 1.0)
    }

    pub fn down() -> Vector2 {
        Vector2::new(0.0, -1.0)
    }

    pub fn add(&self, adder: &Vector2) -> Vector2 {
        Vector2::new(self.x + adder.x, self.y + adder.y)
    }

    pub fn sub(&self, subber: &Vector2) -> Vector2 {
        Vector2::new(self.x - subber.x, self.y - subber.y)
    }
}
