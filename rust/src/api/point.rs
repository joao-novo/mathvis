//! Module containing a point implementation and its operations
#![warn(missing_docs)]
use std::error::Error;
use std::ops::{Add, Sub};

use rand::distr::{Distribution, StandardUniform};
use rand::rng;

use super::util::Number;
use super::vector::Vector;

/// Trait that defines behavior similar to a point.
///
/// A PointLike is anything that can be represented on an n-dimensional coordinate system, and has a position.
pub trait PointLike<T: Number> {
    /// Creates a new PointLike with the specified coordinates.
    /// Returns an Option because the arguments are not always expected to be valid.
    fn new(values: Vec<T>) -> Option<Self>
    where
        Self: Sized;
    /// Creates a new PointLike with coordinates at the origin, on a coordinate system of the specified dimension.
    /// Returns an Option since the dimension has to be greater than 0.
    fn origin(dimensions: u32) -> Option<Self>
    where
        Self: Sized;

    /// Creates a PointLike of the specified dimensions, with random coordinates.
    ///
    /// Returns an Option vecause the dimension has to be greater than 0.
    fn random(dimensions: u32) -> Option<Self>
    where
        Self: Sized,
        StandardUniform: Distribution<T>;

    /// Returns a reference to the vector containing the coordinates of the PointLike.
    fn values(&self) -> &Vec<T>;

    /// Returns the dimensions of the PointLike.
    fn get_dimensions(&self) -> usize;
}

/// An n-dimensional point.
///
/// This implementation is generic over any signed number type which implements the traits defined in [Number]
///
/// A point can be compared for equality and the (common equality properties hold)[crate::api::matrix::Matrix]
///
/// # Examples
///
/// ```
/// use mathvis::api::point::{Point, PointLike};
///
/// let p = Point::new(vec![1, 1]).unwrap();
/// p.get_dimensions();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point<T: Number> {
    values: Vec<T>,
}

impl<T> From<imageproc::point::Point<T>> for Point<T>
where
    T: Number,
{
    /// Function that allows conversion from an imageproc Point to this implementation of Point.
    ///
    /// Used for convenience in the internal API and should not be used.
    fn from(value: imageproc::point::Point<T>) -> Self {
        Point::new(vec![value.x, value.y]).unwrap()
    }
}

impl<T> Point<T>
where
    Point<T>: PointLike<T>,
    T: Number + Sub<T, Output = T>,
{
    /// Calculates the distance between two points.
    ///
    /// Returns an Err if the dimensions of the points are different and an Ok with the distance otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::Point;
    /// use mathvis::api::point::PointLike;
    ///
    /// let point = Point::<i32>::origin(2).unwrap();
    /// let other = Point::new(vec![1, 0]).unwrap();
    /// assert_eq!(point.distance_to(&other).unwrap(), 1);
    /// ```
    pub fn distance_to(&self, other: &Point<T>) -> Result<T, Box<dyn Error>> {
        if self.get_dimensions() != other.get_dimensions() {
            return Err("Wrong dimensions.".into());
        }
        Ok(self
            .values
            .iter()
            .zip(other.values.iter())
            .fold(T::zero(), |acc, (a, b)| {
                acc + (a.clone() - b.clone()).pow(2)
            })
            .sqrt())
    }
}

impl<T, U> Add<Vector<U>> for Point<T>
where
    Point<T>: PointLike<T>,
    Vector<U>: PointLike<U>,
    T: Number + Add<U, Output = U>,
    U: Number,
{
    type Output = Result<Point<U>, Box<dyn Error>>;

    /// Adds a vector and a point.
    ///
    /// Returns an Err if the dimensions of the point and the vector are different and an Ok with the resulting point otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::Point;
    /// use mathvis::api::point::PointLike;
    /// use mathvis::api::vector::Vector;
    ///
    /// let point = Point::<i32>::origin(2).unwrap();
    /// let vector = Vector::new(vec![1, 0]).unwrap();
    /// assert_eq!((point + vector).unwrap(), Point::new(vec![1, 0]).unwrap());
    /// ```
    fn add(self, vec: Vector<U>) -> Self::Output {
        if self.get_dimensions() != vec.get_dimensions() {
            return Err("Wrong dimensions.".into());
        }
        Ok(Point {
            values: self
                .values
                .iter()
                .zip(vec.values().iter())
                .map(|(a, b)| a.clone() + b.clone())
                .collect(),
        })
    }
}

impl<T> PointLike<T> for Point<T>
where
    T: Number,
{
    /// Creates a new Point with the specified coordinates.
    ///
    /// Returns a None if the coordinates vector is empty and a Some with the Point otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::{Point, PointLike};
    ///
    /// let p1 = Point::<i32>::new(Vec::new());
    /// let p2 = Point::new(vec![1, 1]);
    ///
    /// assert!(p1 == None && p2 == Some(Point::new(vec![1, 1]).unwrap()));
    /// ```
    fn new(values: Vec<T>) -> Option<Self>
    where
        Self: Sized,
    {
        if values.is_empty() {
            return None;
        }
        Some(Point { values })
    }

    /// Creates a new Point on the origin with the specified dimensions.
    ///
    /// Returns a None if the dimension is 0 and a Some with the Point otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::{Point, PointLike};
    ///
    /// let p = Point::<i32>::origin(2);
    ///
    /// assert!(p == Some(Point::new(vec![0, 0]).unwrap()));
    /// ```
    fn origin(dimensions: u32) -> Option<Self> {
        if dimensions == 0 {
            return None;
        }
        Some(Point {
            values: vec![T::zero(); dimensions as usize],
        })
    }

    /// Returns a reference to a vector with the point's coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::{Point, PointLike};
    ///
    /// let p = Point::<i32>::origin(2).unwrap();
    ///
    /// assert!(p.values() == &vec![0, 0]);
    /// ```
    fn values(&self) -> &Vec<T> {
        &self.values
    }

    /// Returns the point's dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::{Point, PointLike};
    ///
    /// let p = Point::<i32>::origin(5).unwrap();
    ///
    /// assert!(p.get_dimensions() == 5);
    /// ```
    fn get_dimensions(&self) -> usize {
        self.values.len()
    }

    /// Creates a point with the specified dimensions and random coordinates.
    /// Not meant to be used for anything other than testing purposes.
    ///
    /// Returns a None if the dimension is 0 and a Some with the point otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::{Point, PointLike};
    /// let p = Point::<i32>::random(4).unwrap();
    /// ```
    fn random(dimensions: u32) -> Option<Self>
    where
        Self: Sized,
        StandardUniform: Distribution<T>,
    {
        if dimensions == 0 {
            return None;
        }

        let mut rng = rng();
        Some(Point {
            values: (0..dimensions)
                .map(|_| StandardUniform.sample(&mut rng))
                .collect(),
        })
    }
}
