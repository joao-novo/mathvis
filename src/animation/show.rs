//! Module containing a trait definition for showable objects.
#![warn(missing_docs)]
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use imageproc::image::{Rgb, RgbImage};

use crate::api::{matrix::Matrix, point::Point, screen::Screen2D, util::Number};

/// Trait representing a showable object.
/// A Show2D object can be contained by a [Screen2D], and can be shown on the screen and moved around.
pub trait Show2D<T>
where
    T: Number,
{
    /// Returns the x coordinate of the object.
    fn x(&self) -> T;
    /// Returns the y coordinate of the object.
    fn y(&self) -> T;
    /// Adds a context to the object. Necessary for it to be shown on screen.
    ///
    /// Returns an Err if the object cannot be contained by the [Screen2D] and an Ok otherwise.
    fn add_context(&mut self, context: Arc<Mutex<Screen2D>>) -> Result<(), Box<dyn Error>>;
    /// Draws an object on the specified image with the specified color.
    ///
    /// Returns an Err if the object does not have a context and an Ok otherwise.
    fn draw(&self, color: Rgb<u8>, img: &mut RgbImage) -> Result<(), Box<dyn Error>>;
    /// Moves an object along a parametric function with one parameter, for the specified duration.    
    ///
    /// Returns an Err if the object does not have a context or if anything goes wrong with the animation process and an Ok otherwise.
    fn move_along_parametric<F>(
        &self,
        duration: f32,
        parametric: F,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(f64) -> (f64, f64) + Send + Sync + 'static;
    /// Rotates an object for a specified duration, by a specified angle, on a specified center of rotation.
    ///
    /// Returns an Err if the object does not have a context or if anything goes wrong with the animation process and an Ok otherwise.
    fn rotate(&self, duration: f32, angle: f64, center: Point<f64>) -> Result<(), Box<dyn Error>>;
    /// Moves an object to a specified point, for a specified duration.
    ///
    /// Returns an Err if the object does not have a context or if anything goes wrong with the animation process and an Ok otherwise.
    fn move_to(&self, duration: f32, point: Point<f64>) -> Result<(), Box<dyn Error>>;
    /// Moves an object to the result of its transformation by multiplication by the specified matrix, for a specified duration.
    ///
    /// Returns an Err if the object does not have a context or if anything goes wrong with the animation process and an Ok otherwise.
    fn multiply_by_matrix(&self, duration: f32, matrix: Matrix<T>) -> Result<(), Box<dyn Error>>;
    /// Moves an object to the result of its transformation by multiplication by the specified matrix, for a specified duration,
    /// by separating its rotation and scaling.
    ///
    /// Warning: Currently not working and should not be used.
    ///
    /// Returns an Err if the object does not have a context or if anything goes wrong with the animation process and an Ok otherwise.
    fn rotate_then_scale(&self, duration: f32, matrix: Matrix<T>) -> Result<(), Box<dyn Error>>;
}
