pub trait Move {
    fn move_to(&self, x: f64, y: f64) -> Result<Self, &str>
    where
        Self: Sized;
}
