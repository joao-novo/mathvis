//! Module containing an n-dimensional vector structure and its respective operations.
#![warn(missing_docs)]
use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};

use super::{point::PointLike, util::Number};
use std::{
    error::Error,
    ops::{Add, Mul},
};

/// An n-dimensional vector that allows for different vector operations.
///
/// Vector implements [PointLike], essentially making it behave like a point with special vector-specific operations (dot product, multiplication with matrices).
/// It also implements [Eq] and [PartialEq], so the (common equality properties hold)[crate::api::matrix::Matrix].
///
/// # Examples
///
/// ```
/// use mathvis::api::vector::Vector;
/// use mathvis::api::point::PointLike;
///
/// let v = Vector::new(vec![1, 0]).unwrap();
/// v.norm();
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Vector<T: Number> {
    pub(crate) values: Vec<T>,
}

impl<T> Vector<T>
where
    T: Number,
    Vector<T>: PointLike<T>,
{
    /// Calculates the dot product of two vectors.
    ///
    /// Returns an Err if the vectors have different dimensions and an Ok with the result otherwise.
    /// Both vectors must be of the same type and the result is always of that type. This second condition should never be an issue since the dot product of two vectors with integers is always an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::vector::Vector;
    /// use mathvis::api::point::PointLike;
    ///
    /// let v1 = Vector::new(vec![1, 1]).unwrap();
    /// let v2 = Vector::new(vec![1, 1]).unwrap();
    /// assert_eq!(v1.dot(v2).unwrap(), 2);
    /// ```
    pub fn dot(&self, rhs: Vector<T>) -> Result<T, Box<dyn Error>> {
        if self.get_dimensions() != rhs.get_dimensions() {
            return Err("wrong dimensions".into());
        }
        Ok(self
            .values
            .iter()
            .zip(rhs.values.iter())
            .fold(T::zero(), |acc, (a, b)| acc + *a * *b))
    }

    /// Calculates the norm of a vector.
    /// The norm is always of the same type as the vector, so it may lead to rounding when using integer vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::vector::Vector;
    /// use mathvis::api::point::PointLike;
    ///
    /// let vector = Vector::new(vec![1, 0]).unwrap();
    /// assert_eq!(vector.norm(), 1);
    /// ```
    pub fn norm(&self) -> T {
        self.values
            .iter()
            .fold(T::zero(), |acc, a| acc + *a * *a)
            .sqrt()
    }

    /// Normalizes a vector.
    /// The resulting vector is always of the same type as the original vector, so be careful when using integer vectors.
    ///
    /// Returns an Err if the norm is 0, since that would cause division by zero, and an Ok with the resulting vector otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::vector::Vector;
    /// use mathvis::api::point::PointLike;
    ///
    /// let vector = Vector::new(vec![2, 0]).unwrap();
    /// assert_eq!(vector.normalize().unwrap(), Vector::new(vec![1, 0]).unwrap());
    /// ```
    pub fn normalize(&self) -> Result<Vector<T>, Box<dyn Error>> {
        if self.norm() == T::zero() {
            return Err("Cannot normalize vector of norm 0".into());
        }
        Ok(Vector {
            values: self.values.iter().map(|val| *val / self.norm()).collect(),
        })
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

    /// Adds two vectors together according to regular vector addition.
    /// Both vectors can be of different types but the resulting vector will always be of the second one's type, and addition between floats and integers is not allowed.
    ///
    /// Returns an Err if the dimensions are different and an Ok with the resulting vector otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::vector::Vector;
    /// use mathvis::api::point::PointLike;
    /// let v1 = Vector::new(vec![1, 1]).unwrap();
    /// assert!((v1.clone() + v1).unwrap() == Vector::new(vec![2, 2]).unwrap());
    /// ```
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

    /// Performs a cross product between two 3D vectors.
    ///
    /// Returns Err if both vectors' dimensions are not 3 and an Ok with the result otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::{point::PointLike, vector::Vector};
    ///
    /// let v1 = Vector::new(vec![1, 0, 1]).unwrap();
    /// assert!((v1.clone() * v1).unwrap() == Vector::new(vec![0, 0, 0]).unwrap());
    /// ```
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

    /// Multiplies a vector by a scalar.
    /// The result will always be a vector of the type of the scalar.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::{point::PointLike, vector::Vector};
    ///
    /// let v1 = Vector::new(vec![1, 1]).unwrap();
    ///
    /// assert!(v1.clone() * 2 == Vector::new(vec![2, 2]).unwrap());
    /// ```
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
    /// Creates a new Vector with the specified values.
    ///
    /// Returns None if the coordinates vector is empty and a Some otherwise.
    ///
    /// # Examples
    /// ```
    /// use mathvis::api::{point::PointLike, vector::Vector};
    ///
    /// let v1 = Vector::new(vec![1, 1]);
    /// let v2 = Vector::<f32>::new(Vec::new());
    ///
    /// assert!(v1 == Some(Vector::new(vec![1, 1]).unwrap()) && v2 == None);
    /// ```
    fn new(values: Vec<T>) -> Option<Self>
    where
        Self: Sized,
    {
        if values.is_empty() {
            return None;
        }
        Some(Vector { values })
    }

    /// Creates a new Vector on the origin with the specified dimensions.
    ///
    /// Returns a None if the dimension is 0 and a Some with the Vector otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::PointLike;
    /// use mathvis::api::vector::Vector;
    ///
    /// let v = Vector::<i32>::origin(2);
    ///
    /// assert!(v == Some(Vector::new(vec![0, 0]).unwrap()));
    /// ```
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

    /// Returns a reference to the vector's coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::PointLike;
    /// use mathvis::api::vector::Vector;
    ///
    /// let v = Vector::<i32>::origin(2).unwrap();
    ///
    /// assert!(v.values() == &vec![0, 0]);
    /// ```
    fn values(&self) -> &Vec<T> {
        &self.values
    }

    /// Returns the vector's dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::PointLike;
    /// use mathvis::api::vector::Vector;
    ///
    /// let v = Vector::<i32>::origin(5).unwrap();
    ///
    /// assert!(v.get_dimensions() == 5);
    /// ```
    fn get_dimensions(&self) -> usize {
        self.values.len()
    }

    /// Creates a vector with the specified dimensions and random coordinates.
    /// Not meant to be used for anything other than testing purposes.
    ///
    /// Returns a None if the dimension is 0 and a Some with the vector otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::point::PointLike;
    /// use mathvis::api::vector::Vector;
    /// let v = Vector::<i32>::random(4).unwrap();
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
        assert!(a.dot(b).unwrap() == 10.0);
    }

    #[test]
    fn test_cross() {
        let a: Vector<f32> = Vector::new(vec![1.0, 2.0, 3.0]).unwrap();
        let b: Vector<f32> = Vector::new(vec![3.0, 2.0, 1.0]).unwrap();
        let c: Vector<f32> = Vector::new(vec![-4.0, 8.0, -4.0]).unwrap();
        assert!(a * b == Ok(c));
    }
}
