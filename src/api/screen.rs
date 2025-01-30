use super::{point::PointLike, util::in_axis_range};

pub trait ScreenLike {
    fn can_contain<T: PointLike>(&self, object: &T) -> Result<bool, &str>;
    fn x_axis(&self) -> (f64, f64);
    fn y_axis(&self) -> (f64, f64);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Screen2D {
    x_axis: (f64, f64),
    y_axis: (f64, f64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Screen3D {
    x_axis: (f64, f64),
    y_axis: (f64, f64),
    z_axis: (f64, f64),
}

impl Screen2D {
    pub fn new((xstart, xend): (f64, f64), (ystart, yend): (f64, f64)) -> Option<Self> {
        if xstart < xend && ystart < yend {
            return Some(Screen2D {
                x_axis: (xstart, xend),
                y_axis: (ystart, yend),
            });
        }
        None
    }
}

impl ScreenLike for Screen2D {
    fn x_axis(&self) -> (f64, f64) {
        self.x_axis
    }

    fn y_axis(&self) -> (f64, f64) {
        self.y_axis
    }

    fn can_contain<T: PointLike>(&self, object: &T) -> Result<bool, &str> {
        match object.get_dimensions() {
            2 => Ok(in_axis_range(object.value()[0], self.x_axis)
                && in_axis_range(object.value()[1], self.y_axis)),
            _ => Err("wrong dimensions"),
        }
    }
}

impl Screen3D {
    pub fn new(
        (xstart, xend): (f64, f64),
        (ystart, yend): (f64, f64),
        (zstart, zend): (f64, f64),
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

    pub fn z_axis(&self) -> (f64, f64) {
        self.z_axis
    }
}

impl ScreenLike for Screen3D {
    fn x_axis(&self) -> (f64, f64) {
        self.x_axis
    }

    fn y_axis(&self) -> (f64, f64) {
        self.y_axis
    }

    fn can_contain<T: PointLike>(&self, object: &T) -> Result<bool, &str> {
        match object.get_dimensions() {
            3 => Ok(in_axis_range(object.value()[0], self.x_axis)
                && in_axis_range(object.value()[0], self.y_axis)
                && in_axis_range(object.value()[2], self.z_axis)),
            _ => Err("wrong dimensions"),
        }
    }
}
