//! A module containing a 2D and later on, a 3D screen that holds global properties of the program.
#![warn(missing_docs)]
use std::{error::Error, f32};

use crate::animation::show::Show2D;

use super::{
    point::{Point, PointLike},
    util::{in_axis_range, Number},
};

/// Trait that defines behavior belonging to a screen.
///
/// A ScreenLike is anything that contains at least 2 axes and can contain objects that are [Show2D] or Show3D, which are essentially 2D (and later on 3D) wrappers for [PointLike] objects.
pub trait ScreenLike<V: Number> {
    /// Checks whether or not a certain [Show2D] or Show3D-implementing object can be contained in the axes of the screen.
    /// Currently only the first case is implemented since there are no 3D objects yet.
    fn can_contain<T: Show2D<V>>(&self, object: &T) -> bool;
    /// Returns two floats containing the minimum and maximum value of the x axis.
    fn x_axis(&self) -> (f32, f32);
    /// Returns two floats containing the minimum and maximum value of the y axis.
    fn y_axis(&self) -> (f32, f32);
}

/// A 2D screen, with several global properties.
///
/// This implementation implements [PartialEq], meaning the common equality properties hold, except for the reflexive property (there's no big reason why it shouldn't have this, but having it would require using integers for the axis limits).
///
/// # Examples
///
/// ```
/// use mathvis::api::screen::{Screen2D, ScreenLike};
/// let s = Screen2D::new((-10.0, 10.0), (-10.0, 10.0), String::from("./save"), 30, 1920, 1080).unwrap();
///
/// <Screen2D as ScreenLike<f32>>::x_axis(&s);
/// ```
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

impl Screen2D {
    /// Creates a new screen with the specified axes, save directory, fps, width and height.
    ///
    /// Returns a None if the axes limits are not valid (end > start) and a Some with the Screen otherwise.
    /// Warning this function is not meant to be used directly as the Screen2D is created automatically, and only its axes may be changed through `change_dimensions`
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::screen::Screen2D;
    ///
    /// let s1 = Screen2D::new((10.0, 5.0), (-10.0, 10.0), String::from("./save"), 30, 1920, 1080);
    /// let s2 = Screen2D::new((-10.0, 10.0), (-10.0, 10.0), String::from("./save"), 30, 1920, 1080);
    /// assert!(s1 == None && s2 == Some(Screen2D::new((-10.0, 10.0), (-10.0, 10.0), String::from("./save"), 30, 1920, 1080).unwrap()));
    /// ```
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

    /// Changes the axes' limits to the specified ones.
    ///
    /// Returns an Err if the specified dimensions are invalid and an Ok otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::screen::{ScreenLike, Screen2D};
    ///
    /// let mut screen = Screen2D::new((-10.0, 10.0), (-10.0, 10.0), String::from("./save"), 30, 1920, 1080).unwrap();
    /// screen.change_dimensions((-5.0, 5.0), (-5.0, 5.0));
    /// assert_eq!(<Screen2D as ScreenLike<f32>>::x_axis(&screen), (-5.0, 5.0));
    /// ```
    pub fn change_dimensions(
        &mut self,
        (xstart, xend): (f32, f32),
        (ystart, yend): (f32, f32),
    ) -> Result<(), Box<dyn Error>> {
        if xstart < xend && ystart < yend {
            self.x_axis = (xstart, xend);
            self.y_axis = (ystart, yend);
            return Ok(());
        }
        Err("Invalid axes' dimensions.".into())
    }

    /// Returns the position of the origin in pixels.
    ///
    /// The pixel count starts on the top left corner and goes down and right for the y and x axis respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::screen::{Screen2D, ScreenLike};
    /// let mut screen = Screen2D::new((-10.0, 10.0), (-10.0, 10.0), String::from("./save"), 30, 1920, 1080).unwrap();
    /// assert!(screen.get_center_pixels() == (960.0, 540.0));
    /// ```
    pub fn get_center_pixels(&self) -> (f32, f32) {
        let ratio_x = self.x_axis.0.abs() / (self.x_axis.1.abs() + self.x_axis.0.abs());
        let ratio_y = self.y_axis.1.abs() / (self.y_axis.1.abs() + self.y_axis.0.abs());
        (self.width as f32 * ratio_x, self.height as f32 * ratio_y)
    }

    /// Updates the current frame value to a specified value.
    /// Not meant to be used outside of internal API
    ///
    /// Returns an Err if the specified frame value is not greater than the current one and an Ok otherwise.
    pub(crate) fn change_current_frame(&mut self, val: u32) -> Result<(), Box<dyn Error>> {
        if val > self.current_frame {
            self.current_frame = val;
            return Ok(());
        }
        Err("You can't change the frame to an earlier one.".into())
    }
}

impl<T: Number> ScreenLike<T> for Screen2D {
    /// Returns the x axis limits of the screen
    fn x_axis(&self) -> (f32, f32) {
        self.x_axis
    }

    /// Returns the y axis limits of the screen
    fn y_axis(&self) -> (f32, f32) {
        self.y_axis
    }

    /// Returns true if the specified object can be contained by the screen, that is, if the object's coordinates are in the axes' range.
    fn can_contain<V>(&self, object: &V) -> bool
    where
        V: Show2D<T>,
    {
        in_axis_range(object.x(), self.x_axis) && in_axis_range(object.y(), self.y_axis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center() {
        let screen =
            Screen2D::new((-10.0, 10.0), (-10.0, 15.0), String::new(), 30, 1920, 1080).unwrap();
        assert!(screen.get_center_pixels() == (960.0, 648.0));
    }
}
