use std::{f32, path::PathBuf};

use crate::animation::show::Show2D;

use super::{
    point::{Point, PointLike},
    util::{in_axis_range, Number},
};

pub trait ScreenLike<V: Number> {
    fn can_contain<T: Show2D<V>>(&self, object: &T) -> bool;
    fn x_axis(&self) -> (f32, f32);
    fn y_axis(&self) -> (f32, f32);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Screen2D {
    x_axis: (f32, f32),
    y_axis: (f32, f32),
    pub(crate) save_directory: String,
    pub(crate) current_frame: u32,
    pub(crate) fps: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Screen3D {
//     x_axis: (f64, f64),
//     y_axis: (f64, f64),
//     z_axis: (f64, f64),
// }

impl Screen2D {
    pub fn new(
        (xstart, xend): (f32, f32),
        (ystart, yend): (f32, f32),
        save_directory: String,
        fps: u32,
        width: u32,
        height: u32,
    ) -> Option<Self> {
        if xstart < xend && ystart < yend {
            return Some(Screen2D {
                x_axis: (xstart, xend),
                y_axis: (ystart, yend),
                save_directory,
                current_frame: 0,
                fps,
                width,
                height,
            });
        }
        None
    }

    pub fn change_dimensions(&mut self, (xstart, xend): (f32, f32), (ystart, yend): (f32, f32)) {
        if xstart < xend && ystart < yend {
            self.x_axis = (xstart, xend);
            self.y_axis = (ystart, yend);
        }
    }

    pub fn get_center_pixels(&self, res: Point<f32>) -> (f32, f32) {
        let ratio_x = self.x_axis.0.abs() / (self.x_axis.1.abs() + self.x_axis.0.abs());
        let ratio_y = self.y_axis.1.abs() / (self.y_axis.1.abs() + self.y_axis.0.abs());
        (
            res.values()[0] * ratio_x as f32,
            res.values()[1] * ratio_y as f32,
        )
    }

    pub fn change_current_frame(&mut self, val: u32) {
        if val > self.current_frame {
            self.current_frame = val;
        }
    }
}

impl<T: Number> ScreenLike<T> for Screen2D {
    fn x_axis(&self) -> (f32, f32) {
        self.x_axis
    }

    fn y_axis(&self) -> (f32, f32) {
        self.y_axis
    }

    fn can_contain<V>(&self, object: &V) -> bool
    where
        V: Show2D<T>,
    {
        in_axis_range(object.x(), self.x_axis) && in_axis_range(object.y(), self.y_axis)
    }
}

// impl Screen3D {
//     pub fn new(
//         (xstart, xend): (f64, f64),
//         (ystart, yend): (f64, f64),
//         (zstart, zend): (f64, f64),
//     ) -> Option<Self> {
//         if xstart < xend && ystart < yend && zstart < zend {
//             return Some(Screen3D {
//                 x_axis: (xstart, xend),
//                 y_axis: (ystart, yend),
//                 z_axis: (zstart, zend),
//             });
//         }
//         None
//     }

//     pub fn z_axis(&self) -> (f64, f64) {
//         self.z_axis
//     }
// }

// impl ScreenLike for Screen3D {
//     fn x_axis(&self) -> (f64, f64) {
//         self.x_axis
//     }

//     fn y_axis(&self) -> (f64, f64) {
//         self.y_axis
//     }

//     fn can_contain<T: Show2D<T>>(&self, object: &T) -> bool {
//         in_axis_range(object.value()[0], self.x_axis)
//             && in_axis_range(object.value()[0], self.y_axis)
//             && in_axis_range(object.value()[2], self.z_axis)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center() {
        let screen =
            Screen2D::new((-10.0, 10.0), (-10.0, 15.0), String::new(), 30, 1920, 1080).unwrap();
        println!(
            "{:?}",
            screen.get_center_pixels(Point::new(vec![1920.0, 1080.0]).unwrap())
        );
        assert!(
            screen.get_center_pixels(Point::new(vec![1920.0, 1080.0]).unwrap()) == (960.0, 648.0)
        );
    }
}
