use super::{motion::Move, point::Point2DLike, screen::Screen2D};
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2D<'a> {
    context: &'a Screen2D,
    x: f64,
    y: f64,
}

impl<'a> Vector2D<'a> {
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<'a> Add for Vector2D<'a> {
    type Output = Result<Vector2D<'a>, &'a str>;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(vector) = Self::new(self.context, self.x + rhs.x, self.y + rhs.y) {
            return Ok(vector);
        };
        Err("out of bounds")
    }
}

impl<'a> Point2DLike<'a> for Vector2D<'a> {
    fn is_within_context(context: &'a Screen2D, x: f64, y: f64) -> bool {
        x >= 0. && x < context.width() as f64 && y >= 0. && y < context.height() as f64
    }

    fn new(context: &'a Screen2D, x: f64, y: f64) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::is_within_context(context, x, y) {
            return Some(Vector2D { context, x, y });
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

impl<'a> Move for Vector2D<'a> {
    fn move_to(&self, x: f64, y: f64) -> Result<Self, &str>
    where
        Self: Sized,
    {
        if let Some(vector) = Self::new(self.context, x, y) {
            return Ok(vector);
        }
        Err("out of bounds")
    }
}
