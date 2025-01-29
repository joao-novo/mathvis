use super::{motion::Move, screen::Screen2D, vector::Vector2D};

pub trait Point2DLike<'a> {
    fn is_within_context(context: &'a Screen2D, x: f64, y: f64) -> bool;
    fn new(context: &'a Screen2D, x: f64, y: f64) -> Option<Self>
    where
        Self: Sized;
    fn origin(context: &'a Screen2D) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<'a> {
    context: &'a Screen2D,
    x: f64,
    y: f64,
}

impl<'a> Point2D<'a> {
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn add_vector(&self, vec: Vector2D) -> Result<Self, &str>
    where
        Self: Sized,
    {
        if let Some(point) = Self::new(self.context, self.x + vec.x(), self.y + vec.y()) {
            return Ok(point);
        }
        Err("out of bounds")
    }
}

impl<'a> Point2DLike<'a> for Point2D<'a> {
    fn is_within_context(context: &'a Screen2D, x: f64, y: f64) -> bool {
        x >= 0. && x < context.width() as f64 && y >= 0. && y < context.height() as f64
    }

    fn new(context: &'a Screen2D, x: f64, y: f64) -> Option<Point2D<'a>> {
        if Self::is_within_context(context, x, y) {
            return Some(Point2D { context, x, y });
        }
        None
    }

    fn origin(context: &'a Screen2D) -> Self {
        Self::new(context, 0.0, 0.0).unwrap()
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl<'a> Move for Point2D<'a> {
    fn move_to(&self, x: f64, y: f64) -> Result<Self, &str>
    where
        Self: Sized,
    {
        if let Some(point) = Self::new(self.context, x, y) {
            return Ok(point);
        }
        Err("out of bounds")
    }
}
