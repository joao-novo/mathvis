use clap::ValueEnum;

use super::screen::{Screen2D, ScreenLike};
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
    pub fn resolution(&self) -> (f32, f32) {
        match self {
            Quality::LOW => (854.0, 480.0),
            Quality::MEDIUM => (1280.0, 720.0),
            Quality::HIGH => (1920.0, 1080.0),
            Quality::ULTRA => (3840.0, 2160.0),
        }
    }

    pub fn usable(&self) -> (f32, f32) {
        let (w, h) = self.resolution();
        ((w - 0.05 * w as f32), (h - 0.05 * h as f32))
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
    let (sw, sh) = quality.resolution();
    let (usable_sw, usable_sh) = quality.usable();
    let center = screen.get_center_pixels((sw, sh));
    let scaling_factor = (
        usable_sw as f32 / (screen.x_axis().0.abs() + screen.x_axis().1.abs()),
        usable_sh as f32 / (screen.y_axis().0.abs() + screen.y_axis().1.abs()),
    );
    (
        x * scaling_factor.0 + center.0,
        -y * scaling_factor.1 + center.1,
    )
}
