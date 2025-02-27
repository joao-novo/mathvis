use clap::ValueEnum;
use num_traits::{Num, ToPrimitive};

use super::{
    point::{Point, PointLike},
    screen::{Screen2D, ScreenLike},
};
pub fn in_axis_range(val: f32, (start, end): (f32, f32)) -> bool {
    start <= val && val <= end
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Quality {
    LOW,
    MEDIUM,
    HIGH,
    ULTRA,
}

impl Quality {
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

pub fn interpolate(quality: &Quality, screen: &Screen2D, (x, y): (f32, f32)) -> (f32, f32) {
    let res = quality.resolution();
    let usable_res = quality.usable();
    let center =
        screen.get_center_pixels(Point::new(vec![res.values()[0], res.values()[1]]).unwrap());
    let scaling_factor = (
        usable_res.values()[0] / (screen.x_axis().0.abs() + screen.x_axis().1.abs()),
        usable_res.values()[1] / (screen.y_axis().0.abs() + screen.y_axis().1.abs()),
    );
    (
        x * scaling_factor.0 + center.0,
        -y * scaling_factor.1 + center.1,
    )
}

pub trait Number: Num + Clone + Copy + ToPrimitive {}

impl<T: Num + ToPrimitive + Copy + Clone> Number for T {}
