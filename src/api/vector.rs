use super::{motion::Move, point::Point2DLike, screen::Screen};

pub struct Vector2D<'a> {
    context: &'a Screen,
    x: f64,
    y: f64,
}

impl<'a> Point2DLike<'a> for Vector2D<'a> {
    fn is_within_context(context: &'a Screen, x: f64, y: f64) -> bool {
        x >= 0. && x < context.width() as f64 && y >= 0. && y < context.height() as f64
    }

    fn new(context: &'a Screen, x: f64, y: f64) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::is_within_context(context, x, y) {
            return Some(Vector2D { context, x, y });
        }
        None
    }

    fn origin(context: &'a Screen) -> Self {
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
        if let Some(point) = Self::new(self.context, x, y) {
            return Ok(point);
        }
        Err("out of bounds")
    }
}
