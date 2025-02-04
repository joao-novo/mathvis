use std::ops::Add;

use rand::{rng, Rng};

use super::vector::Vector;

pub trait PointLike {
    fn new(values: Vec<f64>) -> Option<Self>
    where
        Self: Sized;
    fn origin(dimensions: u32) -> Option<Self>
    where
        Self: Sized;
    fn random(dimensions: u32) -> Option<Self>
    where
        Self: Sized;
    fn value(&self) -> &Vec<f64>;

    fn get_dimensions(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    values: Vec<f64>,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> Result<f64, &str> {
        if self.get_dimensions() != other.get_dimensions() {
            return Err("wrong dimensions");
        }
        Ok(self
            .values
            .iter()
            .zip(other.values.iter())
            .fold(0.0, |acc, (a, b)| acc + (a - b) * (a - b))
            .sqrt())
    }
}

impl Add<Vector> for Point {
    type Output = Result<Point, String>;

    fn add(self, vec: Vector) -> Self::Output {
        if self.get_dimensions() != vec.get_dimensions() {
            return Err(String::from("wrong dimensions"));
        }
        Ok(Point {
            values: self
                .values
                .iter()
                .zip(vec.value().iter())
                .map(|(a, b)| a + b)
                .collect(),
        })
    }
}

impl PointLike for Point {
    fn new(values: Vec<f64>) -> Option<Self>
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
            values: vec![0.0; dimensions as usize],
        })
    }

    fn value(&self) -> &Vec<f64> {
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
            values: (0..dimensions).map(|_| rng.random()).collect(),
        })
    }
}

// impl<'a> Move for Point2D<'a> {
//     fn move_to(&self, x: f64, y: f64) -> Result<Self, &str>
//     where
//         Self: Sized,
//     {
//         if let Some(point) = Self::new(self.context, x, y) {
//             return Ok(point);
//         }
//         Err("out of bounds")
//     }
// }
