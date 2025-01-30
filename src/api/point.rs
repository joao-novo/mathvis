use super::vector::Vector;

pub trait PointLike {
    fn new(values: Vec<f64>) -> Option<Self>
    where
        Self: Sized;
    fn origin(dimensions: u32) -> Option<Self>
    where
        Self: Sized;
    fn value(&self) -> &Vec<f64>;

    fn get_dimensions(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    values: Vec<f64>,
    //TODO implement context for 2d and 3d points
    context: Option<i32>,
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

    pub fn add_vector(&self, vec: Vector) -> Result<Self, &str>
    where
        Self: Sized,
    {
        if self.get_dimensions() != vec.get_dimensions() {
            return Err("wrong dimensions");
        }
        Ok(Point {
            values: self
                .values
                .iter()
                .zip(vec.value().iter())
                .map(|(a, b)| a + b)
                .collect(),
            context: None,
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
        Some(Point {
            values,
            context: None,
        })
    }

    fn origin(dimensions: u32) -> Option<Self> {
        if dimensions == 0 {
            return None;
        }
        Some(Point {
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
