//! Module containing a simple implementation of a matrix and its respective operations
#![warn(missing_docs)]
use std::{
    error::Error,
    iter::Sum,
    ops::{AddAssign, Mul},
};

use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};

use super::{
    point::PointLike,
    util::{quadsolve, Number},
    vector::Vector,
};

/// A matrix with any width or length implemented using a vector of vectors
///
/// This matrix implementation is generic over any type of number (for simplicity's sake,
/// unsigned number types must be converted into signed types, might be changed soon) which implements
/// the traits defined in [Number].
///
/// A matrix implements PartialEq and Eq which means they can be compared for equality and
/// the reflexive property holds.
///
/// ```text
/// m1 == m2 <==> m2 == m1
/// m1 == m2 && m2 == m3 <==> m1 == m3
/// ```
///
/// # Examples
///
/// ```
/// use mathvis::api::matrix::Matrix;
/// // All functions that return a new matrix (except operations that return a matrix) are wrapped in an Option unless
/// // it's absolutely guaranteed that they always return a valid matrix.
/// if let Some(matrix) = Matrix::<f32>::identity(2) {
///
/// }
/// ```
///
/// ```
/// use mathvis::api::matrix::Matrix;
/// if let Some(matrix) = Matrix::<f32>::identity(2) {
///     // All matrix operations have their result wrapped in a Result unless
///     // they're guaranteed to work
///     matrix.determinant().unwrap();
/// }
///
/// ```
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Matrix<T: Number> {
    pub(crate) values: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Number,
{
    /// Creates a new Matrix with the specified values.
    ///
    /// Returns a None if the vector is empty or if all rows are not the same length.
    /// Returns a Some with the matrix otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// let m1 = Matrix::<i32>::new(Vec::new());
    /// let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    ///
    /// assert!(m1 == None && m2 == Some(Matrix::new(vec![vec![1, 1], vec![1, 1]]).unwrap()));
    /// ```
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

    /// Creates an identity matrix with the specified dimensions. By definition,
    /// it's always a square matrix so its size is n x n where n is the specified dimension.
    ///
    /// Returns a None if the dimension is 0 and a Some with the matrix otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// let m = Matrix::<i32>::identity(0);
    ///
    /// assert!(m == None);
    /// ```
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

    /// Creates a 2d rotation matrix that when applied to a vector (see [Vector2D](crate::animation::vector::Vector2D)), rotates it by the specified angle in radians.
    /// around the origin.
    ///
    /// Since there is no way this method would not work, it always returns a matrix, and for
    /// simplicity's sake, the matrix always contains f32 values, since it works by calculating
    /// trigonometric functions of an f32.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// use std::f32::consts::PI;
    /// let m = Matrix::<f32>::rotation_matrix_2d(PI); // creates a matrix that rotates a vector by PI radians
    /// ```
    pub fn rotation_matrix_2d(angle: f32) -> Matrix<f32> {
        Matrix {
            values: vec![
                vec![angle.cos(), -angle.sin()],
                vec![angle.sin(), angle.cos()],
            ],
        }
    }

    /// Creates a random matrix of the specified dimensions.
    /// Not meant to be used for anything other than testing purposes.
    ///
    /// Returns None if the number of rows or columns is 0 and Some with the matrix otherwise.
    fn random((rows, cols): (usize, usize)) -> Option<Self>
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

    /// Returns the dimensions of this matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    ///
    /// let matrix = Matrix::<i32>::identity(2).unwrap();
    /// assert_eq!(matrix.get_dimensions(), (2, 2));
    /// ```
    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.values.len(), self.values[0].len())
    }

    /// Calculates the determinant of the matrix using the definition.
    ///
    /// Returns a Result, returning an Err if the matrix is not square and an Ok otherwise.
    /// Warning: currently not very efficient and may be changed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// let m = Matrix::<f32>::identity(2).unwrap();
    ///
    /// assert!(m.determinant().unwrap() == 1.0);
    /// ```
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

    /// Transposes a matrix.
    ///
    /// Since transposing a matrix works for any type of matrix, it doesn't return an Option or a Result.
    /// Instead, it just returns the matrix itself.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// let m = Matrix::new(vec![vec![1, 0], vec![1, 1]]).unwrap();
    /// assert!(m.transpose() == Matrix::new(vec![vec![1, 1], vec![0, 1]]).unwrap());
    /// ```
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

    /// Calculates and returns the eigenvalues of a 2x2 matrix.
    /// Uses the quadratic formula to calculate the zeroes of the characteristic polynomial.
    ///
    /// Returns an Err if the matrix is not 2x2 and an Ok with the values otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    ///
    /// let matrix = Matrix::<f32>::identity(2).unwrap();
    /// assert_eq!(matrix.eigenvalues_2d().unwrap(), (1.0, 1.0));
    /// ```
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

    /// Calculates and returns the eigenvectors of a 2x2 matrix.
    ///
    /// Returns an Err if the matrix is not 2x2 and an Ok with the vectors otherwise.
    ///
    /// Warning: Doesn't work with matrices that have only one distinct eigenvalue currently
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// use mathvis::api::vector::Vector;
    /// use mathvis::api::point::PointLike;
    ///
    /// let matrix = Matrix::new(vec![vec![2, 1], vec![1, 2]]).unwrap();
    /// assert_eq!(matrix.eigenvectors_2d().unwrap(), (Vector::new(vec![1, 1]).unwrap(), Vector::new(vec![-1, 1]).unwrap()));
    /// ```
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

    /// Calculates and returns the inverse of a 2x2 matrix.
    ///
    /// Doesn't work if the matrix has determinant 0.
    /// Returns an Err if the matrix is not 2x2 and an Ok with the inverted matrix otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    ///
    /// let matrix = Matrix::<i32>::identity(2).unwrap();
    /// assert_eq!(matrix.invert_2d().unwrap(), Matrix::<i32>::identity(2).unwrap());
    /// ```
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

    /// Performs Singular Value Decomposition on a 2x2 matrix.
    ///
    /// SVD is a similar process to diagonalization, but it's performed on A^T A, and the diagonal matrix
    /// contains the singular values, which are the square root of the eigenvalues.
    ///
    /// Returns an Err if the matrix is not 2x2 and an Ok with the matrices U, Sigma and V inside otherwise.
    pub fn svd_2d(self) -> Result<(Matrix<T>, Matrix<T>, Matrix<T>), Box<dyn Error>> {
        let transpose_a_by_a = (self.transpose() * self.clone())?;
        if let Ok((l1, l2)) = transpose_a_by_a.clone().eigenvalues_2d() {
            let sigma =
                Matrix::new(vec![vec![l1.sqrt(), T::zero()], vec![T::zero(), l2.sqrt()]]).unwrap();
            let (v1, v2) = transpose_a_by_a.eigenvectors_2d()?;
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

    /// Performs polar decomposition of a 2x2 matrix.
    ///
    /// This process consists in the separation of a matrix in a rotation and scaling matrix, using SVD.
    /// Warning: Currently doesn't work properly.
    ///
    /// Returns an Err if the matrix is not 2x2 and an Ok with both the rotation and scaling matrices otherwise.
    pub fn polar_decomposition_2d(self) -> Result<(Matrix<T>, Matrix<T>), Box<dyn Error>> {
        if let Ok((u, sigma, v)) = self.clone().svd_2d() {
            let s = ((u * sigma)? * v)?;
            let q = (self * s.clone().invert_2d()?)?;
            return Ok((q, s));
        }
        Err("Matrix is not 2x2".into())
    }
}

impl<T, U> Mul<Matrix<U>> for Matrix<T>
where
    T: Number + AddAssign<T> + Mul<U, Output = T>,
    U: Number + Mul<T, Output = U>,
{
    type Output = Result<Matrix<T>, Box<dyn Error>>;

    /// Multiplies two matrices together.
    ///
    /// Returns an Err if the dimensions aren't fit for matrix multiplication and an Ok with the result otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// let m1 = Matrix::new(vec![vec![1, 1], vec![1, 1]]).unwrap();
    /// let i = Matrix::<i32>::identity(2).unwrap();
    ///
    /// assert_eq!((m1.clone() * i).unwrap(), m1);
    /// ```
    fn mul(self, rhs: Matrix<U>) -> Self::Output {
        if self.get_dimensions().1 != rhs.get_dimensions().0 {
            return Err("Wrong dimensions.".into());
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

    /// Scales a matrix by a scalar value.
    ///
    /// Returns a matrix of the type of the scalar instead of the original type.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// let m1 = Matrix::<f32>::identity(2).unwrap();
    /// assert!(m1 * 2.0 == Matrix::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]]).unwrap());
    /// ```
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

    /// Multiplies a vector by a matrix.
    ///
    /// Returns an Err if the matrix height is not the same as the vector's dimension and an Ok with the result otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathvis::api::matrix::Matrix;
    /// use mathvis::api::vector::Vector;
    /// use mathvis::api::point::PointLike;
    ///
    /// let m1 = Matrix::<i32>::identity(2).unwrap();
    /// let v = Vector::random(2).unwrap();
    ///
    /// assert!((m1 * v.clone()).unwrap() == v);
    /// ```
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
        let b = Matrix::random((2, 2)).unwrap();
        assert!((a * b.clone()).unwrap() == b);
    }

    #[test]
    fn test_multiply() {
        let a = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]).unwrap();
        let b = Matrix::new(vec![vec![1.0, 1.0], vec![0.0, 1.0]]).unwrap();
        let c = Matrix::new(vec![vec![1.0, 2.0], vec![1.0, 2.0]]).unwrap();
        assert!((a * b).unwrap() == c);
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
