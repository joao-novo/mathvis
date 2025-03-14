//! Module containing a 2D wrapper for a vector that implements animation operations
#![warn(missing_docs)]
use std::{
    error::Error,
    f64::consts::PI,
    ops::{Add, Mul},
    sync::{Arc, Mutex},
};

use imageproc::{
    drawing::{draw_line_segment_mut, draw_polygon_mut},
    image::{Rgb, RgbImage},
    point::Point,
};

use crate::{
    api::{
        matrix::Matrix,
        point::{self, PointLike},
        screen::{Screen2D, ScreenLike},
        util::{interpolate, Number, Quality},
        vector::Vector,
    },
    misc::thread_pool::ThreadPool,
};

use super::{axis::draw_axis, background::fill_background, show::Show2D};

/// A 2D vector implementation.
/// Implements some of the operations of [Vector] and contains one inside for access to more general operations.
/// Cannot be compared due to using an Arc for thread safety.
///
/// # Examples
/// ```
/// use mathvis::animation::vector::Vector2D;
/// use imageproc::image::Rgb;
///
/// let vector = Vector2D::new(1, 1, Rgb([255, 255, 255]));
///
/// vector.clone().dot(vector);
/// ```
#[derive(Debug, Clone)]
pub struct Vector2D<T: Number> {
    vector: Vector<T>,
    x: T,
    y: T,
    context: Option<Arc<Mutex<Screen2D>>>,
    color: Rgb<u8>,
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

    fn add_context(&mut self, context: Arc<Mutex<Screen2D>>) -> Result<(), Box<dyn Error>> {
        let context_lock = context.lock().unwrap();
        if !context_lock.can_contain(self) {
            return Err("Vector cannot be contained within the context's bounds.".into());
        }
        self.context = Some(context.clone());
        Ok(())
    }

    fn move_along_parametric<F>(
        &self,
        duration: f32,
        parametric: F,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>>
    where
        F: (Fn(f64) -> (f64, f64)) + Send + Sync + 'static,
    {
        let context = self
            .context
            .clone()
            .ok_or("This object does not have an associated context")?;

        let (current_frame, save_directory, fps, img_width, img_height) = {
            let context_lock = context.lock().map_err(|_| "Failed to lock context")?;
            (
                context_lock.current_frame,
                context_lock.save_directory.clone(),
                context_lock.fps,
                context_lock.width,
                context_lock.height,
            )
        };

        let frames: u32 = (duration * fps as f32) as u32;
        let completed_frames = Arc::new(Mutex::new(0));
        let shared_parametric = Arc::new(parametric);
        let color = Arc::new(self.color);
        let error_flag = Arc::new(Mutex::new(false));

        {
            let thread_pool = ThreadPool::new(fps as usize).unwrap();

            for i in 0..frames {
                let completed_frames = Arc::clone(&completed_frames);
                let error_flag = Arc::clone(&error_flag);
                let context = Arc::clone(&context);
                let save_directory = save_directory.clone();
                let shared_parametric = Arc::clone(&shared_parametric);
                let color = Arc::clone(&color);
                let white = Rgb([255, 255, 255]);

                let frame_generator = move || {
                    let mut img = RgbImage::new(img_width, img_height);

                    let t = t_min + (i as f64 / (frames - 1) as f64) * (t_max - t_min);
                    let (x, y) = shared_parametric(t);

                    let context_lock = match context.lock() {
                        Ok(lock) => lock,
                        Err(_) => {
                            let mut error = error_flag.lock().unwrap();
                            *error = true;
                            return;
                        }
                    };

                    fill_background(&mut img);
                    draw_axis(&mut img, white, Arc::new(context_lock.clone()));

                    drop(context_lock);

                    let mut v = Vector2D::new(x, y, *color);
                    if let Err(_) = v.add_context(context.clone()) {
                        let mut error = error_flag.lock().unwrap();
                        *error = true;
                        return;
                    }

                    if let Err(_) = v.draw(v.color, &mut img) {
                        let mut error = error_flag.lock().unwrap();
                        *error = true;
                        return;
                    }
                    match img.save(format!(
                        "{}/tmp/frame_{:03}.png",
                        save_directory,
                        current_frame + i,
                    )) {
                        Ok(_) => {
                            let mut completed = completed_frames.lock().unwrap();
                            *completed += 1;
                            println!("Generated frame {}", current_frame + i);
                        }
                        Err(_) => {
                            let mut error = error_flag.lock().unwrap();
                            *error = true;
                        }
                    }
                };

                thread_pool.execute(frame_generator);
            }
        }

        let completed = *completed_frames.lock().unwrap();
        let has_error = *error_flag.lock().unwrap();

        if has_error || completed != frames as usize {
            return Err(format!(
                "Frame generation failed. Completed: {}, Total: {}",
                completed, frames
            )
            .into());
        }

        {
            let mut context_lock = context.lock().unwrap();
            context_lock
                .change_current_frame(current_frame + frames)
                .unwrap();
        }

        Ok(())
    }

    fn rotate(
        &self,
        duration: f32,
        angle: f64,
        center: point::Point<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let (x, y) = (Arc::new(self.x), Arc::new(self.y));
        self.move_along_parametric(
            duration,
            move |t| {
                (
                    (Arc::clone(&x).to_f64() - center.values()[0]) * t.cos()
                        - (Arc::clone(&y).to_f64() - center.values()[1]) * t.sin()
                        + center.values()[0],
                    (Arc::clone(&x).to_f64() - center.values()[0]) * angle.sin()
                        + (Arc::clone(&y).to_f64() - center.values()[1]) * t.cos()
                        + center.values()[1],
                )
            },
            0.0,
            angle,
        )
    }
    fn move_to(&self, duration: f32, point: point::Point<f64>) -> Result<(), Box<dyn Error>> {
        let (x, y) = (Arc::new(self.x), Arc::new(self.y));
        self.move_along_parametric(
            duration,
            move |t| {
                (
                    (1.0 - t) * x.to_f64() + t * point.values()[0],
                    (1.0 - t) * y.to_f64() + t * point.values()[1],
                )
            },
            0.0,
            1.0,
        )
    }

    fn multiply_by_matrix(&self, duration: f32, matrix: Matrix<T>) -> Result<(), Box<dyn Error>> {
        let vector = (matrix * self.clone()).unwrap();
        self.move_to(
            duration,
            point::Point::new(vec![vector.x.to_f64(), vector.y.to_f64()]).unwrap(),
        )?;
        Ok(())
    }

    fn rotate_then_scale(&self, duration: f32, matrix: Matrix<T>) -> Result<(), Box<dyn Error>> {
        let (q, s) = matrix.polar_decomposition_2d()?;
        println!("{:?} {:?}", q.clone(), s.clone());
        self.multiply_by_matrix(duration / 2.0, q.clone())?;
        let mid = (q * self.clone())?;
        mid.multiply_by_matrix(duration / 2.0, s)?;
        Ok(())
    }
}

impl<T: Number> Vector2D<T> {
    /// Creates a new Vector2D with the specified x, y and color.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::animation::vector::Vector2D;
    /// use imageproc::image::Rgb;
    ///
    /// let color = Rgb([255, 0, 255]);
    /// let vector = Vector2D::new(5, 2, color);
    /// ```
    pub fn new(x: T, y: T, color: Rgb<u8>) -> Self {
        // Known to work since x and y always exist
        let vector = Vector::new(vec![x, y]).unwrap();
        Self {
            vector,
            x,
            y,
            context: None,
            color,
        }
    }

    /// Wrapper for the dot product between two vectors.
    /// Unlike its more general counterpart, since both vectors always have the same dimension, it returns the value itself.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::animation::vector::Vector2D;
    /// use imageproc::image::Rgb;
    ///
    /// let vector = Vector2D::new(1, 1, Rgb([255, 255, 255]));
    /// assert_eq!(vector.clone().dot(vector), 2);
    /// ```
    pub fn dot(&self, other: Vector2D<T>) -> T {
        // Always works because both vectors are 2D
        self.vector.dot(other.vector).unwrap()
    }

    /// Creates a new Vector2D on the origin with the specified color.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::animation::vector::Vector2D;
    /// use imageproc::image::Rgb;
    ///
    /// let color = Rgb([255, 0, 255]);
    /// let vector = Vector2D::<f32>::origin(color);
    /// ```
    pub fn origin(color: Rgb<u8>) -> Self {
        let vector = Vector::origin(2).unwrap();
        Self {
            vector,
            x: T::zero(),
            y: T::zero(),
            context: None,
            color,
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
        if self
            .context
            .as_ref()
            .zip(rhs.context.as_ref())
            .map_or(false, |(a, b)| *a.lock().unwrap() != *b.lock().unwrap())
        {
            return Err("LHS and RHS don't share the same context.".into());
        }
        Ok(Self {
            vector,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            context: self.context,
            color: self.color,
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
            color: self.color,
        };
    }
}

impl<T: Number> Mul<Vector2D<T>> for Matrix<T> {
    type Output = Result<Vector2D<T>, Box<dyn Error>>;

    fn mul(self, rhs: Vector2D<T>) -> Self::Output {
        if self.get_dimensions() != (2, 2) {
            return Err("Matrix must be 2x2 to apply to a 2d vector.".into());
        }
        let vals = self.values;
        let (x, y) = (
            vals[0][0] * rhs.x + vals[0][1] * rhs.y,
            vals[1][0] * rhs.x + vals[1][1] * rhs.y,
        );
        Ok(Vector2D {
            vector: Vector::new(vec![x, y]).unwrap(),
            x,
            y,
            context: rhs.context,
            color: rhs.color,
        })
    }
}

pub(crate) fn draw_vector<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: Arc<Mutex<Screen2D>>,
) where
    T: Number,
{
    let screen = screen.lock().unwrap();
    let quality = Quality::new(img.width(), img.height()).unwrap();
    let center = screen.get_center_pixels();
    let (x, y) = interpolate(
        quality,
        Arc::new(screen.clone()),
        (
            vector.values()[0].to_f64() as f32,
            vector.values()[1].to_f64() as f32,
        ),
    );
    draw_line_segment_mut(img, center, (x, y), color);
    draw_vector_tip(vector, img, color, Arc::new(screen.clone()), quality);
}

fn rotate(point: &Point<f64>, angle: f64, rotation_center: &Point<f64>) -> Point<f64> {
    let new_x = (point.x - rotation_center.x) * angle.cos()
        - (point.y - rotation_center.y) * angle.sin()
        + rotation_center.x;
    let new_y = (point.x - rotation_center.x) * angle.sin()
        + (point.y - rotation_center.y) * angle.cos()
        + rotation_center.y;
    Point::new(new_x, new_y)
}

fn draw_vector_tip<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: Arc<Screen2D>,
    quality: Quality,
) where
    T: Number,
{
    let (a, b) = (vector.values()[0].to_f64(), vector.values()[1].to_f64());
    let (p1, p2): (point::Point<f64>, point::Point<f64>) = (
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
    let (x, y) = interpolate(quality.clone(), screen.clone(), (a as f32, b as f32));
    let (x1, y1) = interpolate(
        quality.clone(),
        screen.clone(),
        (p1.values()[0] as f32, p1.values()[1] as f32),
    );
    let (x2, y2) = interpolate(
        quality,
        screen,
        (p2.values()[0] as f32, p2.values()[1] as f32),
    );

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
