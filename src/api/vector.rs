//! Module containing an n-dimensional vector structure and its respective operations.
//! Will eventually be used with a special wrapper for displayable vectors
#![warn(missing_docs)]
use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};

use super::{point::PointLike, util::Number};
use std::ops::{Add, Mul};

/// An n-dimensional vector that allows for different vector operations.
///
/// Vector implements [`PointLike`](trait.PointLike.html), essentially making it behave like a point with special vector-specific operations (dot product, multiplication with matrices).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Vector<T: Number> {
    values: Vec<T>,
}

impl<T> Vector<T>
where
    T: Number,
    Vector<T>: PointLike<T>,
{
    pub fn dot(&self, rhs: Vector<impl Number>) -> Result<f32, &str> {
        if self.get_dimensions() != rhs.get_dimensions() {
            return Err("wrong dimensions");
        }
        Ok(self
            .values
            .iter()
            .zip(rhs.values.iter())
            .fold(0.0, |acc, (a, b)| {
                acc + a.to_f32().unwrap() * b.to_f32().unwrap()
            }))
    }
}

impl<T, U> Add<Vector<U>> for Vector<T>
where
    T: Number + Add<U, Output = U>,
    U: Number,
    Vector<T>: PointLike<T>,
    Vector<U>: PointLike<U>,
{
    type Output = Result<Vector<U>, String>;

    fn add(self, rhs: Vector<U>) -> Self::Output {
        if self.get_dimensions() != rhs.get_dimensions() {
            return Err(String::from("wrong dimensions"));
        }
        Ok(Vector {
            values: self
                .values
                .iter()
                .zip(rhs.values.iter())
                .map(|(a, b): (&T, &U)| a.clone() + b.clone())
                .collect(),
        })
    }
}

impl<T, U> Mul<Vector<U>> for Vector<T>
where
    T: Number + Mul<U, Output = U>,
    U: Number,
    Vector<T>: PointLike<T>,
    Vector<U>: PointLike<U>,
{
    type Output = Result<Vector<U>, String>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        if self.get_dimensions() != rhs.get_dimensions() || self.get_dimensions() != 3 {
            return Err(String::from("wrong dimensions"));
        }
        let (l1, l2, l3) = (
            self.values()[1] * rhs.values()[2] - self.values()[2] * rhs.values()[1],
            self.values()[2] * rhs.values()[0] - self.values()[0] * rhs.values()[2],
            self.values()[0] * rhs.values()[1] - self.values()[1] * rhs.values()[0],
        );
        Ok(Vector {
            values: vec![l1, l2, l3],
        })
    }
}

impl<T, U> Mul<U> for Vector<T>
where
    T: Number,
    U: Number + Mul<T, Output = U>,
{
    type Output = Vector<U>;

    fn mul(self, scalar: U) -> Self::Output {
        Vector {
            values: self.values.iter().map(|val| scalar * val.clone()).collect(),
        }
    }
}

impl<T> PointLike<T> for Vector<T>
where
    T: Number,
{
    fn new(values: Vec<T>) -> Option<Self>
    where
        Self: Sized,
    {
        if values.is_empty() {
            return None;
        }
        Some(Vector { values })
    }

    fn origin(dimensions: u32) -> Option<Self>
    where
        Self: Sized,
    {
        if dimensions == 0 {
            return None;
        }
        Some(Vector {
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
        StandardUniform: Distribution<T>,
    {
        if dimensions == 0 {
            return None;
        }

        let mut rng = rng();
        Some(Vector {
            values: (0..dimensions).map(|_| rng.random()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let a = Vector {
            values: vec![1.0, 2.0, 3.0],
        };
        let b = Vector {
            values: vec![0.0, 1.0, 0.0],
        };
        let c = Vector {
            values: vec![1.0, 3.0, 3.0],
        };

        assert!(a + b == Ok(c));
    }

    #[test]
    fn test_add_wrong_dimensions() {
        let a = Vector {
            values: vec![1.0, 2.0, 3.0],
        };
        let b = Vector::<f32>::random(2).unwrap();
        assert!(a + b == Err(String::from("wrong dimensions")));
    }

    #[test]
    fn test_dot() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]).unwrap();
        let b = Vector::new(vec![3.0, 2.0, 1.0]).unwrap();
        assert!(a.dot(b) == Ok(10.0));
    }

    #[test]
    fn test_cross() {
        let a: Vector<f32> = Vector::new(vec![1.0, 2.0, 3.0]).unwrap();
        let b: Vector<f32> = Vector::new(vec![3.0, 2.0, 1.0]).unwrap();
        let c: Vector<f32> = Vector::new(vec![-4.0, 8.0, -4.0]).unwrap();
        assert!(a * b == Ok(c));
    }
}
