use std::ops::Mul;

use rand::{rng, Rng};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    values: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(values: Vec<Vec<f32>>) -> Option<Self> {
        let first_length = values.first().map_or(0, |row| row.len());
        if values.len() == 0
            || first_length == 0
            || !values.iter().all(|row| row.len() == first_length)
        {
            return None;
        }
        Some(Matrix { values })
    }

    pub fn identity(dimensions: usize) -> Option<Self> {
        if dimensions == 0 {
            return None;
        }
        Some(Matrix {
            values: (0..dimensions)
                .map(|i| {
                    let mut row = vec![0.0; dimensions];
                    row[i] = 1.0;
                    row
                })
                .collect(),
        })
    }

    pub fn random_matrix((rows, cols): (usize, usize)) -> Option<Self> {
        if rows == 0 || cols == 0 {
            return None;
        }
        let mut rng = rng();
        let vals: Vec<Vec<f32>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.random()).collect())
            .collect();
        Some(Matrix { values: vals })
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.values.len(), self.values[0].len())
    }

    pub fn determinant(&self) -> Result<f32, &str> {
        if self.dimensions().0 != self.dimensions().1 {
            return Err("must be a square matrix");
        }
        let size = self.dimensions().0;
        if size == 1 {
            return Ok(self.values[0][0]);
        }

        let mut curr_determinant = 0.0;
        for col in 0..size {
            let value = self.values[0][col];
            let mut sub_values: Vec<Vec<f32>> = Vec::new();
            for row in 1..size {
                let mut sub_row_values: Vec<f32> = Vec::new();
                for collumn in 0..size {
                    if collumn != col {
                        sub_row_values.push(self.values[row][collumn]);
                    }
                }
                sub_values.push(sub_row_values);
            }

            //TODO error handling
            let sub_matrix = Matrix::new(sub_values).unwrap();
            curr_determinant +=
                (if col % 2 == 0 { 1.0 } else { -1.0 }) * value * sub_matrix.determinant().unwrap();
        }

        Ok(curr_determinant)
    }
}

impl Mul for Matrix {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.dimensions().1 != rhs.dimensions().0 {
            return Err(String::from("wrong dimensions"));
        }
        let (a, b) = (&self.values, &rhs.values);
        let (n, p) = (self.dimensions().0, rhs.dimensions().1);
        let mut c = vec![vec![0.0; p]; n];
        for i in 0..n {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..self.dimensions().1 {
                    sum += a[i][k] * b[k][j]
                }
                c[i][j] = sum;
            }
        }
        Ok(Matrix { values: c })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let vals: Vec<Vec<f32>> = Vec::new();
        assert!(Matrix::new(vals) == None);
    }

    #[test]
    fn test_different_lengths() {
        let vals = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0]];
        assert!(Matrix::new(vals) == None);
    }

    #[test]
    fn test_multiply_identity() {
        let a = Matrix::identity(2).unwrap();
        let b = Matrix::random_matrix((2, 2)).unwrap();
        assert!(a * b.clone() == Ok(b));
    }

    #[test]
    fn test_multiply() {
        let a = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]).unwrap();
        let b = Matrix::new(vec![vec![1.0, 1.0], vec![0.0, 1.0]]).unwrap();
        let c = Matrix::new(vec![vec![1.0, 2.0], vec![1.0, 2.0]]).unwrap();
        assert!(a * b == Ok(c));
    }

    #[test]
    fn test_determinant() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0],
            vec![1.0, 2.0, 2.0],
        ])
        .unwrap();
        assert!(a.determinant() == Ok(4.0));
    }
}
