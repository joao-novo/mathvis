use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use imageproc::image::{Rgb, RgbImage};

use crate::api::{screen::Screen2D, util::Number};

pub trait Show2D<T>
where
    T: Number,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn add_context(&mut self, context: Arc<Screen2D>) -> Result<(), Box<dyn Error>>;
    fn draw(&self, color: Rgb<u8>, img: &mut RgbImage) -> Result<(), Box<dyn Error>>;
    fn move_along_parametric<F>(
        &self,
        color: Rgb<u8>,
        img: &mut RgbImage,
        duration: u32,
        parametric: F,
        t_min: f32,
        t_max: f32,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(f32) -> (f32, f32);
}
