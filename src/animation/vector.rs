use std::{
    error::Error,
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use imageproc::{
    drawing::{draw_line_segment_mut, draw_polygon_mut},
    image::{Rgb, RgbImage},
    point::Point,
};

use crate::api::{
    point::{self, PointLike},
    screen::{Screen2D, ScreenLike},
    util::{interpolate, Global, Number, Quality},
    vector::Vector,
};

use super::show::Show2D;

#[derive(Debug, Clone)]
pub struct Vector2D<T: Number> {
    vector: Vector<T>,
    x: T,
    y: T,
    context: Option<Arc<Mutex<Global>>>,
}

impl<T: Number> Show2D<T> for Vector2D<T> {
    fn x(&self) -> T {
        return self.x;
    }

    fn y(&self) -> T {
        return self.y;
    }

    fn draw(self, color: Rgb<u8>) -> Result<(), Box<dyn Error>> {
        println!("test");
        if let Some(context) = self.context {
            let global_lock = context.lock().unwrap();
            let mut image = global_lock
                .current_image
                .lock()
                .map_err(|e| format!("Failed to lock mutex: {}", e))?;
            draw_vector(
                &self.vector,
                &mut *image,
                color,
                global_lock.screen.clone(),
                global_lock.quality.clone(),
            );
            return Ok(());
        }
        Err(
            "This object does not have an associated context. Try using the add_context method."
                .into(),
        )
    }

    fn add_context(&mut self, context: Global) -> Result<(), Box<dyn Error>> {
        if !context.screen.can_contain(self) {
            return Err("vector cannot be contained by context".into());
        }
        self.context = Some(Arc::new(Mutex::new(context.clone())));
        Ok(())
    }
}

impl<T: Number> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        // Known to work since x and y always exist
        let vector = Vector::new(vec![x, y]).unwrap();
        Self {
            vector,
            x,
            y,
            context: None,
        }
    }

    pub fn dot(&self, other: Vector2D<impl Number>) -> f32 {
        // Always works because both vectors are 2D
        self.vector.dot(other.vector).unwrap()
    }
}

pub(crate) fn draw_vector<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: Arc<Screen2D>,
    quality: Arc<Quality>,
) where
    T: Number,
{
    let center = screen.get_center_pixels(quality.resolution());
    let (x, y) = interpolate(
        quality.clone(),
        screen.clone(),
        (
            vector.values()[0].to_f32().unwrap(),
            vector.values()[1].to_f32().unwrap(),
        ),
    );
    draw_line_segment_mut(img, center, (x, y), color);
    draw_vector_tip(vector, img, color, screen, quality);
}

pub(crate) fn rotate(point: &Point<f32>, angle: f32, rotation_center: &Point<f32>) -> Point<f32> {
    let new_x = (point.x - rotation_center.x) * angle.cos()
        - (point.y - rotation_center.y) * angle.sin()
        + rotation_center.x;
    let new_y = (point.x - rotation_center.x) * angle.sin()
        + (point.y - rotation_center.y) * angle.cos()
        + rotation_center.y;
    Point::new(new_x, new_y)
}

pub(crate) fn draw_vector_tip<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: Arc<Screen2D>,
    quality: Arc<Quality>,
) where
    T: Number,
{
    let (a, b) = (
        vector.values()[0].to_f32().unwrap(),
        vector.values()[1].to_f32().unwrap(),
    );
    let (p1, p2): (point::Point<f32>, point::Point<f32>) = (
        rotate(
            &Point::new(a, b),
            2.0 * PI / 3.0,
            &Point::new(0.95 * a, 0.95 * b),
        )
        .into(),
        rotate(
            &Point::new(a, b),
            4.0 * PI / 3.0,
            &Point::new(0.95 * a, 0.95 * b),
        )
        .into(),
    );
    let (x, y) = interpolate(quality.clone(), screen.clone(), (a, b));
    let (x1, y1) = interpolate(
        quality.clone(),
        screen.clone(),
        (p1.values()[0], p1.values()[1]),
    );
    let (x2, y2) = interpolate(quality, screen, (p2.values()[0], p2.values()[1]));

    draw_polygon_mut(
        img,
        &[
            Point::new(x as i32, y as i32),
            Point::new(x1 as i32, y1 as i32),
            Point::new(x2 as i32, y2 as i32),
        ],
        color,
    );
}
