use super::{
    point::{Point, PointLike},
    util::in_axis_range,
};

pub trait ScreenLike {
    // fn can_contain<T: PointLike>(&self, object: &T) -> Result<bool, &str>;
    fn x_axis(&self) -> (f32, f32);
    fn y_axis(&self) -> (f32, f32);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Screen2D {
    x_axis: (f32, f32),
    y_axis: (f32, f32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Screen3D {
    x_axis: (f32, f32),
    y_axis: (f32, f32),
    z_axis: (f32, f32),
}

impl Screen2D {
    pub fn new((xstart, xend): (f32, f32), (ystart, yend): (f32, f32)) -> Option<Self> {
        if xstart < xend && ystart < yend {
            return Some(Screen2D {
                x_axis: (xstart, xend),
                y_axis: (ystart, yend),
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
        (res.values()[0] * ratio_x, res.values()[1] * ratio_y)
    }
}

impl ScreenLike for Screen2D {
    fn x_axis(&self) -> (f32, f32) {
        self.x_axis
    }

    fn y_axis(&self) -> (f32, f32) {
        self.y_axis
    }

    // fn can_contain<T: PointLike>(&self, object: &T) -> Result<bool, &str> {
    //     match object.get_dimensions() {
    //         2 => Ok(in_axis_range(object.value()[0], self.x_axis)
    //             && in_axis_range(object.value()[1], self.y_axis)),
    //         _ => Err("wrong dimensions"),
    //     }
    // }
}

impl Screen3D {
    pub fn new(
        (xstart, xend): (f32, f32),
        (ystart, yend): (f32, f32),
        (zstart, zend): (f32, f32),
    ) -> Option<Self> {
        if xstart < xend && ystart < yend && zstart < zend {
            return Some(Screen3D {
                x_axis: (xstart, xend),
                y_axis: (ystart, yend),
                z_axis: (zstart, zend),
            });
        }
        None
    }

    pub fn z_axis(&self) -> (f32, f32) {
        self.z_axis
    }
}

impl ScreenLike for Screen3D {
    fn x_axis(&self) -> (f32, f32) {
        self.x_axis
    }

    fn y_axis(&self) -> (f32, f32) {
        self.y_axis
    }

    // fn can_contain<T: PointLike>(&self, object: &T) -> Result<bool, &str> {
    //     match object.get_dimensions() {
    //         3 => Ok(in_axis_range(object.value()[0], self.x_axis)
    //             && in_axis_range(object.value()[0], self.y_axis)
    //             && in_axis_range(object.value()[2], self.z_axis)),
    //         _ => Err("wrong dimensions"),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center() {
        let screen = Screen2D::new((-10.0, 10.0), (-10.0, 15.0)).unwrap();
        println!(
            "{:?}",
            screen.get_center_pixels(Point::new(vec![1920.0, 1080.0]).unwrap())
        );
        assert!(
            screen.get_center_pixels(Point::new(vec![1920.0, 1080.0]).unwrap()) == (960.0, 648.0)
        );
    }
}
