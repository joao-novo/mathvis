use num_traits::{Num, ToPrimitive};
use std::ops::{Add, Sub};

use rand::distr::{Distribution, StandardUniform};
use rand::rng;

use super::util::Number;
use super::vector::Vector;

pub trait PointLike<T: Num + Clone + ToPrimitive> {
    fn new(values: Vec<T>) -> Option<Self>
    where
        Self: Sized;
    fn origin(dimensions: u32) -> Option<Self>
    where
        Self: Sized;
    fn random(dimensions: u32) -> Option<Self>
    where
        Self: Sized;
    fn values(&self) -> &Vec<T>;

    fn get_dimensions(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point<T: Number> {
    values: Vec<T>,
}

impl<T> Point<T>
where
    Point<T>: PointLike<T>,
    T: Number + Sub<T, Output = T>,
{
    pub fn distance_to(&self, other: &Point<T>) -> Result<f32, &str> {
        if self.get_dimensions() != other.get_dimensions() {
            return Err("wrong dimensions");
        }
        Ok(self
            .values
            .iter()
            .zip(other.values.iter())
            .fold(0.0, |acc, (a, b)| {
                acc + (a.clone() - b.clone()).to_f32().unwrap().powi(2)
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
    type Output = Result<Point<U>, String>;

    fn add(self, vec: Vector<U>) -> Self::Output {
        if self.get_dimensions() != vec.get_dimensions() {
            return Err(String::from("wrong dimensions"));
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
    StandardUniform: Distribution<T>,
{
    fn new(values: Vec<T>) -> Option<Self>
    where
        Self: Sized,
    {
        if values.is_empty() {
            return None;
        }
        Some(Point { values })
    }

    fn origin(dimensions: u32) -> Option<Self> {
        if dimensions == 0 {
            return None;
        }
        Some(Point {
            values: vec![T::zero(); dimensions as usize],
        })
    }

    fn values(&self) -> &Vec<T> {
        &self.values
    }

    fn get_dimensions(&self) -> usize {
        self.values.len()
    }

    fn random(dimensions: u32) -> Option<Self>
    where
        Self: Sized,
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

// impl<'a> Move for Point2D<'a> {
//     fn move_to(&self, x: f32, y: f32) -> Result<Self, &str>
//     where
//         Self: Sized,
//     {
//         if let Some(point) = Self::new(self.context, x, y) {
//             return Ok(point);
//         }
//         Err("out of bounds")
//     }
// }
