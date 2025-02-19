use rand::{rng, Rng};

use super::point::PointLike;
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    values: Vec<f32>,
}

impl Vector {
    pub fn dot(&self, rhs: Self) -> Result<f32, &str> {
        if self.get_dimensions() != rhs.get_dimensions() {
            return Err("wrong dimensions");
        }
        Ok(self
            .values
            .iter()
            .zip(rhs.values.iter())
            .fold(0.0, |acc, (a, b)| acc + a * b))
    }
}

impl Add for Vector {
    type Output = Result<Vector, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.get_dimensions() != rhs.get_dimensions() {
            return Err(String::from("wrong dimensions"));
        }
        Ok(Vector {
            values: self
                .values
                .iter()
                .zip(rhs.values.iter())
                .map(|(a, b): (&f32, &f32)| a + b)
                .collect(),
        })
    }
}

impl Mul for Vector {
    type Output = Result<Vector, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.get_dimensions() != rhs.get_dimensions() || self.get_dimensions() != 3 {
            return Err(String::from("wrong dimensions"));
        }
        let (l1, l2, l3) = (
            self.value()[1] * rhs.value()[2] - self.value()[2] * rhs.value()[1],
            self.value()[2] * rhs.value()[0] - self.value()[0] * rhs.value()[2],
            self.value()[0] * rhs.value()[1] - self.value()[1] * rhs.value()[0],
        );
        Ok(Vector {
            values: vec![l1, l2, l3],
        })
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            values: rhs.values.iter().map(|val| self * val).collect(),
        }
    }
}

impl PointLike for Vector {
    fn new(values: Vec<f32>) -> Option<Self>
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
            values: vec![0.0; dimensions as usize],
        })
    }

    fn value(&self) -> &Vec<f32> {
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
        Some(Vector {
            values: (0..dimensions).map(|_| rng.random()).collect(),
        })
    }
}

// impl<'a> Move for Vector2D<'a> {
//     fn move_to(&self, x: f32, y: f32) -> Result<Self, &str>
//     where
//         Self: Sized,
//     {
//         if let Some(vector) = Self::new(self.context, x, y) {
//             return Ok(vector);
//         }
//         Err("out of bounds")
//     }
// }
//

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
        let b = Vector::random(2).unwrap();
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
        let a = Vector::new(vec![1.0, 2.0, 3.0]).unwrap();
        let b = Vector::new(vec![3.0, 2.0, 1.0]).unwrap();
        let c = Vector::new(vec![-4.0, 8.0, -4.0]).unwrap();
        assert!(a * b == Ok(c));
    }
}
