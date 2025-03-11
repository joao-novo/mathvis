use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use imageproc::image::{Rgb, RgbImage};

use crate::api::{matrix::Matrix, point::Point, screen::Screen2D, util::Number};

pub trait Show2D<T>
where
    T: Number,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn add_context(&mut self, context: Arc<Mutex<Screen2D>>) -> Result<(), Box<dyn Error>>;
    fn draw(&self, color: Rgb<u8>, img: &mut RgbImage) -> Result<(), Box<dyn Error>>;
    fn move_along_parametric<F>(
        &self,
        duration: f32,
        parametric: F,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(f64) -> (f64, f64) + Send + Sync + 'static;
    fn rotate(&self, duration: f32, angle: f64, center: Point<f64>) -> Result<(), Box<dyn Error>>;
    fn move_to(&self, duration: f32, point: Point<f64>) -> Result<(), Box<dyn Error>>;
    fn multiply_by_matrix(&self, duration: f32, matrix: Matrix<T>) -> Result<(), Box<dyn Error>>;
    fn rotate_then_scale(&self, duration: f32, matrix: Matrix<T>) -> Result<(), Box<dyn Error>>;
}
