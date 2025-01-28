pub struct Point2D(f64, f64);

impl Point2D {
    #[inline]
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D(x, y)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }
}
