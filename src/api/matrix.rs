use std::ops::{AddAssign, Mul, Neg};

use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};

use super::util::Number;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Matrix<T: Number> {
    values: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Number + Neg<Output = T> + AddAssign<T>,
{
    pub fn new(values: Vec<Vec<T>>) -> Option<Self> {
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
                    let mut row = vec![T::zero(); dimensions];
                    row[i] = T::one();
                    row
                })
                .collect(),
        })
    }

    pub fn random_matrix((rows, cols): (usize, usize)) -> Option<Self>
    where
        StandardUniform: Distribution<T>,
    {
        if rows == 0 || cols == 0 {
            return None;
        }
        let mut rng = rng();
        let vals: Vec<Vec<T>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.random()).collect())
            .collect();
        Some(Matrix { values: vals })
    }

    pub fn determinant(&self) -> Result<T, &str> {
        if self.get_dimensions().0 != self.get_dimensions().1 {
            return Err("must be a square matrix");
        }
        let size = self.get_dimensions().0;
        if size == 1 {
            return Ok(self.values[0][0]);
        }

        let mut curr_determinant = T::zero();
        for col in 0..size {
            let value = self.values[0][col];
            let mut sub_values: Vec<Vec<T>> = Vec::new();
            for row in 1..size {
                let mut sub_row_values: Vec<T> = Vec::new();
                for collumn in 0..size {
                    if collumn != col {
                        sub_row_values.push(self.values[row][collumn]);
                    }
                }
                sub_values.push(sub_row_values);
            }

            //TODO error handling
            let sub_matrix = Matrix::new(sub_values).unwrap();
            curr_determinant += (if col % 2 == 0 { T::one() } else { -T::one() })
                * value
                * sub_matrix.determinant().unwrap();
        }

        Ok(curr_determinant)
    }
}

impl<T: Number> Matrix<T> {
    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.values.len(), self.values[0].len())
    }
}

impl<T, U> Mul<Matrix<U>> for Matrix<T>
where
    T: Number + AddAssign<T> + Mul<U, Output = T>,
    U: Number + Mul<T, Output = U>,
{
    type Output = Result<Matrix<T>, String>;

    fn mul(self, rhs: Matrix<U>) -> Self::Output {
        if self.get_dimensions().1 != rhs.get_dimensions().0 {
            return Err(String::from("wrong dimensions"));
        }
        let (a, b) = (&self.values, &rhs.values);
        let (n, p) = (self.get_dimensions().0, rhs.get_dimensions().1);
        let mut c = vec![vec![T::zero(); p]; n];
        for i in 0..n {
            for j in 0..p {
                let mut sum = T::zero();
                for k in 0..self.get_dimensions().1 {
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
        let a: Matrix<f32> = Matrix::identity(2).unwrap();
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
