use std::{
    ops::Neg,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use clap::{command, Parser, ValueEnum};
use imageproc::image::RgbImage;
use num_traits::{Num, ToPrimitive};

use super::{
    point::{Point, PointLike},
    screen::{Screen2D, ScreenLike},
};
pub fn in_axis_range<T: Number>(val: T, (start, end): (f32, f32)) -> bool {
    start <= val.to_f32().unwrap() && val.to_f32().unwrap() <= end
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
                + ScreenLike::<f32>::x_axis(&*screen).1.abs())
            .to_f32()
            .unwrap(),
        usable_res.values()[1]
            / (ScreenLike::<f32>::y_axis(&*screen).0.abs()
                + ScreenLike::<f32>::x_axis(&*screen).1.abs())
            .to_f32()
            .unwrap(),
    );
    (
        x * scaling_factor.0 + center.0,
        -y * scaling_factor.1 + center.1,
    )
}

pub trait Number:
    Num
    + Clone
    + Copy
    + ToPrimitive
    + PartialOrd
    + Neg<Output = Self>
    + PartialEq
    + Send
    + Sync
    + 'static
{
    fn abs(self) -> Self {
        if self > Self::zero() {
            return self;
        }
        -self
    }
}

impl<
        T: Num
            + ToPrimitive
            + Copy
            + Clone
            + PartialOrd
            + Neg<Output = Self>
            + PartialEq
            + Send
            + Sync
            + 'static,
    > Number for T
{
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
