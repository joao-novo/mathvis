use std::{
    error::Error,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg},
};

use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};

use crate::animation::vector::Vector2D;

use super::{
    point::PointLike,
    util::{quadsolve, Number},
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Matrix<T: Number> {
    pub(crate) values: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Number,
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

    pub fn rotation_matrix_2d(angle: f32) -> Matrix<f32> {
        Matrix {
            values: vec![
                vec![angle.cos(), -angle.sin()],
                vec![angle.sin(), angle.cos()],
            ],
        }
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

    pub fn determinant(&self) -> Result<T, Box<dyn Error>> {
        if self.get_dimensions().0 != self.get_dimensions().1 {
            return Err("must be a square matrix".into());
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

    pub fn transpose(&self) -> Matrix<T> {
        let values: Vec<Vec<T>> = (0..self.get_dimensions().1)
            .map(|i| {
                self.values
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<T>>()
            })
            .collect();
        Matrix { values }
    }

    pub fn eigenvalues_2d(self) -> Result<(T, T), Box<dyn Error>> {
        if self.get_dimensions() != (2, 2) {
            return Err("Matrix is not 2x2".into());
        }
        let (a, b, c, d) = (
            self.values[0][0],
            self.values[0][1],
            self.values[1][0],
            self.values[1][1],
        );
        Ok(quadsolve(T::one(), -a - d, -(b * c) + a * d))
    }

    pub fn eigenvectors_2d(self) -> Result<(Vector<T>, Vector<T>), Box<dyn Error>> {
        let (a, b, c, d) = (
            self.values[0][0],
            self.values[0][1],
            self.values[1][0],
            self.values[1][1],
        );
        if let Ok((l1, l2)) = self.eigenvalues_2d() {
            return Ok((
                Vector::new(vec![l1 - a, b]).unwrap().normalize().unwrap(),
                Vector::new(vec![l2 - d, c]).unwrap().normalize().unwrap(),
            ));
        }
        Err("Matrix is not 2x2".into())
    }

    pub fn invert_2d(self) -> Result<Matrix<T>, Box<dyn Error>> {
        if self.get_dimensions() != (2, 2) {
            return Err("Matrix is not 2x2".into());
        }
        let (a, b, c, d) = (
            self.values[0][0],
            self.values[0][1],
            self.values[1][0],
            self.values[1][1],
        );
        Ok(Matrix::new(vec![vec![d, -b], vec![-c, a]]).unwrap()
            * (T::one() / self.determinant()?))
    }

    pub fn svd_2d(self) -> Result<(Matrix<T>, Matrix<T>, Matrix<T>), Box<dyn Error>> {
        if let Ok((l1, l2)) = self.clone().eigenvalues_2d() {
            let sigma =
                Matrix::new(vec![vec![l1.sqrt(), T::zero()], vec![T::zero(), l2.sqrt()]]).unwrap();
            let (v1, v2) = self.eigenvectors_2d()?;
            let u = Matrix::new(vec![
                vec![v1.values()[0], v2.values()[0]],
                vec![v1.values()[1], v2.values()[1]],
            ])
            .unwrap();
            let v = u.clone().invert_2d()?;
            return Ok((u, sigma, v));
        } else {
            Err("Matrix is not 2x2".into())
        }
    }

    pub fn polar_decomposition_2d(self) -> Result<(Matrix<T>, Matrix<T>), Box<dyn Error>> {
        let transpose_a_by_a = (self.transpose() * self.clone())?;
        if let Ok((u, sigma, v)) = transpose_a_by_a.svd_2d() {
            let s = ((u * sigma)? * v)?;
            let q = (self * s.clone().invert_2d()?)?;
            return Ok((q, s));
        }
        Err("Matrix is not 2x2".into())
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

impl<T, U> Mul<U> for Matrix<T>
where
    T: Number + Mul<U, Output = U>,
    U: Number,
{
    type Output = Matrix<U>;

    fn mul(self, scalar: U) -> Self::Output {
        Matrix {
            values: self
                .values
                .iter()
                .map(|row| row.iter().map(|val| val.clone() * scalar).collect())
                .collect(),
        }
    }
}

impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Number + Sum,
{
    type Output = Result<Vector<T>, Box<dyn Error>>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if self.get_dimensions().1 != rhs.get_dimensions() {
            return Err("Matrix must be mxn to multiply by vector of size n.".into());
        }
        Ok(Vector {
            values: self
                .values
                .iter()
                .map(|row| {
                    row.iter()
                        .zip(rhs.values().iter())
                        .map(|(&a, &b)| a * b)
                        .sum()
                })
                .collect(),
        })
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
        assert!(a.determinant().unwrap() == 4.0);
    }

    #[test]
    fn test_matrix_vector_mult() {
        let a = Matrix::new(vec![vec![1, -1, 2], vec![0, -3, 1]]).unwrap();
        let v = Vector::new(vec![2, 1, 0]).unwrap();
        assert!((a * v).unwrap() == Vector::new(vec![1, -3]).unwrap());
    }

    #[test]
    fn test_transpose() {
        let a = Matrix::new(vec![vec![1, 0], vec![1, 1]]).unwrap();
        let v = a.transpose();
        assert!(v == Matrix::new(vec![vec![1, 1], vec![0, 1]]).unwrap());
    }
}
