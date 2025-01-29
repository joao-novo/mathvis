use std::ops::Range;

pub trait Screen2DLike {
    fn x_axis(&self) -> &Range<i32>;
    fn y_axis(&self) -> &Range<i32>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Screen2D {
    x_axis: Range<i32>,
    y_axis: Range<i32>,
}

pub struct Screen3D {
    x_axis: Range<i32>,
    y_axis: Range<i32>,
    z_axis: Range<i32>,
}

impl Screen2D {
    pub fn new((xstart, xend): (i32, i32), (ystart, yend): (i32, i32)) -> Option<Self> {
        if xstart < xend && ystart < yend {
            return Some(Screen2D {
                x_axis: Range {
                    start: xstart,
                    end: xend,
                },
                y_axis: Range {
                    start: ystart,
                    end: yend,
                },
            });
        }
        None
    }
}

impl Screen2DLike for Screen2D {
    fn x_axis(&self) -> &Range<i32> {
        &self.x_axis
    }

    fn y_axis(&self) -> &Range<i32> {
        &self.y_axis
    }
}

impl Screen3D {
    pub fn new(
        (xstart, xend): (i32, i32),
        (ystart, yend): (i32, i32),
        (zstart, zend): (i32, i32),
    ) -> Option<Self> {
        if xstart < xend && ystart < yend {
            return Some(Screen3D {
                x_axis: Range {
                    start: xstart,
                    end: xend,
                },
                y_axis: Range {
                    start: ystart,
                    end: yend,
                },
                z_axis: Range {
                    start: zstart,
                    end: zend,
                },
            });
        }
        None
    }

    pub fn z_axis(&self) -> &Range<i32> {
        &self.z_axis
    }
}

impl Screen2DLike for Screen3D {
    fn x_axis(&self) -> &Range<i32> {
        &self.x_axis
    }

    fn y_axis(&self) -> &Range<i32> {
        &self.y_axis
    }
}
