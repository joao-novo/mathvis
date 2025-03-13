//! Module containing utility functions to be used by the internal API
#![warn(missing_docs)]
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    path::PathBuf,
    sync::Arc,
};

use clap::{command, Parser, ValueEnum};

use super::{
    point::{Point, PointLike},
    screen::{Screen2D, ScreenLike},
};

/// Returns whether or not a value is inside an axis' range.
pub(crate) fn in_axis_range<T: Number>(val: T, (start, end): (f32, f32)) -> bool {
    start <= val.to_f64() as f32 && val.to_f64() as f32 <= end
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) enum Quality {
    LOW,
    MEDIUM,
    HIGH,
    ULTRA,
}

impl Quality {
    /// Creates a new Quality enum instance.
    ///
    /// Returns Some with the value if the x and y values correspond to a valid quality and None otherwise.
    pub(crate) fn new(x: u32, y: u32) -> Option<Quality> {
        match (x, y) {
            (854, 480) => Some(Quality::LOW),
            (1280, 720) => Some(Quality::MEDIUM),
            (1920, 1080) => Some(Quality::HIGH),
            (3840, 2160) => Some(Quality::ULTRA),
            _ => None,
        }
    }

    /// Returns a [Point] with the quality's resolution values.
    pub(crate) fn resolution(&self) -> Point<f32> {
        match self {
            Quality::LOW => Point::new(vec![854.0, 480.0]).unwrap(),
            Quality::MEDIUM => Point::new(vec![1280.0, 720.0]).unwrap(),
            Quality::HIGH => Point::new(vec![1920.0, 1080.0]).unwrap(),
            Quality::ULTRA => Point::new(vec![3840.0, 2160.0]).unwrap(),
        }
    }

    /// Returns a [Point] with the quality's usable resolution values (95% of the total x and y values).
    pub(crate) fn usable(&self) -> Point<f32> {
        let res = self.resolution();
        return Point::new(vec![0.95 * res.values()[0], 0.95 * res.values()[1]]).unwrap();
    }
}

impl ToString for Quality {
    fn to_string(&self) -> String {
        match self {
            Quality::LOW => String::from("low"),
            Quality::MEDIUM => String::from("medium"),
            Quality::HIGH => String::from("high"),
            Quality::ULTRA => String::from("ultra"),
        }
    }
}

/// Converts an (x, y) coordinate into a pixel position.
pub(crate) fn interpolate(
    quality: Quality,
    screen: Arc<Screen2D>,
    (x, y): (f32, f32),
) -> (f32, f32) {
    let usable_res = quality.usable();
    let center = screen.get_center_pixels();
    let scaling_factor = (
        usable_res.values()[0]
            / (ScreenLike::<f32>::x_axis(&*screen).0.abs()
                + ScreenLike::<f32>::x_axis(&*screen).1.abs()),
        usable_res.values()[1]
            / (ScreenLike::<f32>::y_axis(&*screen).0.abs()
                + ScreenLike::<f32>::x_axis(&*screen).1.abs()),
    );
    (
        x * scaling_factor.0 + center.0,
        -y * scaling_factor.1 + center.1,
    )
}

/// Trait that represents a generic signed number type.
/// Number implements all basic operations, partial ordering and equality, Send and Sync for safe passing between threads, Display and Debug for testing purposes, and Sized because all numbers must have a compile-time size
pub trait Number:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + SubAssign
    + MulAssign
    + AddAssign
    + DivAssign
    + Clone
    + Copy
    + PartialOrd
    + PartialEq
    + Send
    + Sync
    + Display
    + std::fmt::Debug
    + Sized
    + 'static
{
    /// Returns the value 0 for that type.
    fn zero() -> Self;
    /// Returns the value 1 for that type.
    fn one() -> Self;
    /// Checks if a value is 0.
    fn is_zero(self) -> bool;
    /// Returns the absolute value of that number.
    fn abs(self) -> Self;
    /// Returns the square root of that number in that type.
    /// For integer types, the result is truncated to only the integer part.
    fn sqrt(self) -> Self;
    /// Returns the result of raising a value to a specified integer.
    fn pow(self, exponent: i32) -> Self;
    /// Converts an f64 into this type.
    fn from_f64(value: f64) -> Self;
    /// Converts an f32 into this type.
    fn from_f32(value: f32) -> Self;
    /// Converts an i64 into this type.
    fn from_i64(value: i64) -> Self;
    /// Converts an i32 into this type.
    fn from_i32(value: i32) -> Self;
    /// Converts this value into an f64
    fn to_f64(self) -> f64;
    /// Converts this value into an i64
    fn to_i64(self) -> i64;
    /// Checks if a value is positive
    fn is_positive(&self) -> bool;
    /// Checks if a value is negative
    fn is_negative(&self) -> bool;
}

impl Number for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn is_zero(self) -> bool {
        self == 0.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn pow(self, exponent: i32) -> Self {
        self.powi(exponent)
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn from_f32(value: f32) -> Self {
        value as f64
    }

    fn from_i64(value: i64) -> Self {
        value as f64
    }

    fn from_i32(value: i32) -> Self {
        value as f64
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn to_i64(self) -> i64 {
        self as i64
    }

    fn is_positive(&self) -> bool {
        *self > 0.0
    }

    fn is_negative(&self) -> bool {
        *self < 0.0
    }
}

impl Number for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn is_zero(self) -> bool {
        self == 0.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn pow(self, exponent: i32) -> Self {
        self.powi(exponent)
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn from_f32(value: f32) -> Self {
        value as f32
    }

    fn from_i64(value: i64) -> Self {
        value as f32
    }

    fn from_i32(value: i32) -> Self {
        value as f32
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn to_i64(self) -> i64 {
        self as i64
    }

    fn is_positive(&self) -> bool {
        *self > 0.0
    }

    fn is_negative(&self) -> bool {
        *self < 0.0
    }
}

impl Number for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn is_zero(self) -> bool {
        self == 0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        (self as f64).sqrt() as i32
    }

    fn pow(self, exponent: i32) -> Self {
        if exponent < 0 {
            return Self::from_f64((self as f64).powi(exponent));
        }
        self.pow(exponent as u32)
    }

    fn from_f64(value: f64) -> Self {
        value as i32
    }

    fn from_f32(value: f32) -> Self {
        value as i32
    }

    fn from_i64(value: i64) -> Self {
        value as i32
    }

    fn from_i32(value: i32) -> Self {
        value
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn to_i64(self) -> i64 {
        self as i64
    }

    fn is_positive(&self) -> bool {
        *self > 0
    }

    fn is_negative(&self) -> bool {
        *self < 0
    }
}

impl Number for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn is_zero(self) -> bool {
        self == 0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        (self as f64).sqrt() as i64
    }

    fn pow(self, exponent: i32) -> Self {
        if exponent < 0 {
            return Self::from_f64((self as f64).powi(exponent));
        }
        self.pow(exponent as u32)
    }

    fn from_f64(value: f64) -> Self {
        value as i64
    }

    fn from_f32(value: f32) -> Self {
        value as i64
    }

    fn from_i64(value: i64) -> Self {
        value
    }

    fn from_i32(value: i32) -> Self {
        value as i64
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn to_i64(self) -> i64 {
        self
    }

    fn is_positive(&self) -> bool {
        *self > 0
    }

    fn is_negative(&self) -> bool {
        *self < 0
    }
}

/// Struct containing the command line arguments for the CLI interface
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub(crate) struct Args {
    pub(crate) source: String,

    #[arg(long, default_value_t = 30)]
    pub(crate) fps: u32,

    #[arg(short, long, default_value_os = "../output/output.mp4")]
    pub(crate) output: PathBuf,

    #[arg(long, default_value_t = false)]
    pub(crate) gif: bool,

    #[arg(short, long, default_value_t = Quality::HIGH)]
    pub(crate) quality: Quality,
}

/// Returns the solution of a quadratic equation with the specified coefficients.
pub(crate) fn quadsolve<T: Number>(a: T, b: T, c: T) -> (T, T) {
    let delta = b * b - a * T::from_f64(4.0) * c;
    (
        (-b + delta.sqrt()) / (a * T::from_f64(2.0)),
        (-b - delta.sqrt()) / (a * T::from_f64(2.0)),
    )
}
