use super::point::PointLike;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    values: Vec<f64>,
    context: Option<i32>,
}

impl Vector {
    pub fn dot(&self, rhs: Self) -> f64 {
        self.values
            .iter()
            .zip(rhs.values.iter())
            .fold(0.0, |acc, (a, b)| acc + a * b)
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
                .map(|(a, b): (&f64, &f64)| a + b)
                .collect(),
            context: None,
        })
    }
}

impl PointLike for Vector {
    fn new(values: Vec<f64>) -> Option<Self>
    where
        Self: Sized,
    {
        if values.is_empty() {
            return None;
        }
        Some(Vector {
            values,
            context: None,
        })
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
            context: None,
        })
    }

    fn value(&self) -> &Vec<f64> {
        &self.values
    }

    fn get_dimensions(&self) -> usize {
        self.values.len()
    }
}

// impl<'a> Move for Vector2D<'a> {
//     fn move_to(&self, x: f64, y: f64) -> Result<Self, &str>
//     where
//         Self: Sized,
//     {
//         if let Some(vector) = Self::new(self.context, x, y) {
//             return Ok(vector);
//         }
//         Err("out of bounds")
//     }
// }
