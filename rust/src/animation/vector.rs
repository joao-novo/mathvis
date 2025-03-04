use std::{
    error::Error,
    f32::consts::PI,
    ops::{Add, Mul},
    sync::{Arc, Mutex},
};

use imageproc::{
    drawing::{draw_line_segment_mut, draw_polygon_mut, Canvas},
    image::{Rgb, RgbImage},
    point::Point,
};

use crate::{
    api::{
        point::{self, PointLike},
        screen::{Screen2D, ScreenLike},
        util::{interpolate, Number, Quality},
        vector::Vector,
    },
    misc::thread_pool::ThreadPool,
};

use super::{axis::draw_axis, background::fill_background, show::Show2D};

#[derive(Debug, Clone)]
pub struct Vector2D<T: Number> {
    vector: Vector<T>,
    x: T,
    y: T,
    context: Option<Arc<Screen2D>>,
}

impl<T: Number> Show2D<T> for Vector2D<T> {
    fn x(&self) -> T {
        return self.x;
    }

    fn y(&self) -> T {
        return self.y;
    }

    fn draw(&self, color: Rgb<u8>, img: &mut RgbImage) -> Result<(), Box<dyn Error>> {
        if let Some(context) = self.clone().context {
            draw_vector(&self.vector, img, color, context.clone());
            return Ok(());
        }
        Err(
            "This object does not have an associated context. Try using the add_context method."
                .into(),
        )
    }

    fn add_context(&mut self, context: Arc<Screen2D>) -> Result<(), Box<dyn Error>> {
        if !context.can_contain(self) {
            return Err("Vector cannot be contained within the context's bounds.".into());
        }
        self.context = Some(context.clone());
        Ok(())
    }

    fn move_along_parametric<F>(
        &self,
        color: Rgb<u8>,
        img: &mut RgbImage,
        duration: u32,
        parametric: F,
        t_min: f32,
        t_max: f32,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(f32) -> (f32, f32),
    {
        if let Some(context) = self.clone().context {
            let current_frame = context.current_frame + 1;
            let frames = duration * context.fps;
            let completed_frames = Arc::new(Mutex::new(0));
            {
                let thread_pool = ThreadPool::new(context.fps as usize).unwrap();
                for i in 0..frames {
                    let completed_frames = Arc::clone(&completed_frames);
                    let mut img = RgbImage::new(img.width(), img.height());
                    let t = t_min + (i as f32 / (frames - 1) as f32) * (t_max - t_min);
                    let (x, y) = parametric(t);
                    let context = context.clone();
                    thread_pool.execute(move || {
                        fill_background(&mut img);
                        draw_axis(&mut img, color, context.clone());
                        let mut v = Vector2D::new(x, y);
                        v.add_context(context.clone()).unwrap();
                        v.draw(color, &mut img).unwrap();
                        img.save(format!(
                            "{}/tmp/frame_{:03}.png",
                            context.save_directory,
                            current_frame + i,
                        ))
                        .unwrap();
                        println!("Generated frame {}", i);
                    })
                }
                // TODO check for completed == done and update context frame count
            }
            return Ok(());
        }
        Err(
            "This object does not have an associated context. Try using the add_context method."
                .into(),
        )
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

    pub fn origin() -> Self {
        let vector = Vector::origin(2).unwrap();
        Self {
            vector,
            x: T::zero(),
            y: T::zero(),
            context: None,
        }
    }
}

impl<T> Add for Vector2D<T>
where
    T: Number,
{
    type Output = Result<Vector2D<T>, Box<dyn Error>>;
    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        let vector = (self.vector + rhs.vector).unwrap();
        if self.context != rhs.context {
            return Err("LHS and RHS don't share the same context.".into());
        }
        Ok(Self {
            vector,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            context: Some(self.context.unwrap().clone()),
        })
    }
}

impl<T, U> Mul<U> for Vector2D<T>
where
    T: Number,
    U: Number + Mul<T, Output = U>,
{
    type Output = Vector2D<U>;

    fn mul(self, scalar: U) -> Self::Output {
        let vector = self.vector * scalar;
        return Vector2D {
            vector,
            x: scalar * self.x,
            y: scalar * self.y,
            context: self.context,
        };
    }
}

pub(crate) fn draw_vector<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: Arc<Screen2D>,
) where
    T: Number,
{
    let quality = Quality::new(img.width(), img.height()).unwrap();
    let center = screen.get_center_pixels(quality.resolution());
    let (x, y) = interpolate(
        quality,
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
    quality: Quality,
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
