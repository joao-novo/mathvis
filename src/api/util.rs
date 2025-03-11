use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use clap::{command, Parser, ValueEnum};
use imageproc::image::RgbImage;
use num_traits::{real::Real, Float, Num, Pow, ToPrimitive};

use super::{
    point::{Point, PointLike},
    screen::{Screen2D, ScreenLike},
};
pub fn in_axis_range<T: Number>(val: T, (start, end): (f32, f32)) -> bool {
    start <= val.to_f64() as f32 && val.to_f64() as f32 <= end
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Quality {
    LOW,
    MEDIUM,
    HIGH,
    ULTRA,
}

impl Quality {
    pub fn new(x: u32, y: u32) -> Option<Quality> {
        match (x, y) {
            (854, 480) => Some(Quality::LOW),
            (1280, 720) => Some(Quality::MEDIUM),
            (1920, 1080) => Some(Quality::HIGH),
            (3840, 2160) => Some(Quality::ULTRA),
            _ => None,
        }
    }
    pub fn resolution(&self) -> Point<f32> {
        match self {
            Quality::LOW => Point::new(vec![854.0, 480.0]).unwrap(),
            Quality::MEDIUM => Point::new(vec![1280.0, 720.0]).unwrap(),
            Quality::HIGH => Point::new(vec![1920.0, 1080.0]).unwrap(),
            Quality::ULTRA => Point::new(vec![3840.0, 2160.0]).unwrap(),
        }
    }

    pub fn usable(&self) -> Point<f32> {
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

pub fn interpolate(quality: Quality, screen: Arc<Screen2D>, (x, y): (f32, f32)) -> (f32, f32) {
    let res = quality.resolution();
    let usable_res = quality.usable();
    let center =
        screen.get_center_pixels(Point::new(vec![res.values()[0], res.values()[1]]).unwrap());
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
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(self) -> bool;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn pow(self, exponent: i32) -> Self;
    fn from_f64(value: f64) -> Self;
    fn from_f32(value: f32) -> Self;
    fn from_i64(value: i64) -> Self;
    fn from_i32(value: i32) -> Self;
    fn to_f64(self) -> f64;
    fn to_i64(self) -> i64;
    fn is_positive(&self) -> bool;
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
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
    pub source: String,

    #[arg(long, default_value_t = 30)]
    pub fps: u32,

    #[arg(short, long, default_value_os = "../output/output.mp4")]
    pub output: PathBuf,

    #[arg(long, default_value_t = false)]
    pub gif: bool,

    #[arg(short, long, default_value_t = Quality::HIGH)]
    pub quality: Quality,
}

pub fn quadsolve<T: Number>(a: T, b: T, c: T) -> (T, T) {
    let delta = b * b - a * T::from_f64(4.0) * c;
    (
        (-b + delta.sqrt()) / (a * T::from_f64(2.0)),
        (-b - delta.sqrt()) / (a * T::from_f64(2.0)),
    )
}
