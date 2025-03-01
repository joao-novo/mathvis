use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use imageproc::image::Rgb;

use crate::api::{
    screen::Screen2D,
    util::{Global, Number},
};

pub trait Show2D<T>
where
    T: Number,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn add_context(&mut self, context: Global) -> Result<(), Box<dyn Error>>;
    fn draw(self, color: Rgb<u8>) -> Result<(), Box<dyn Error>>;
}
