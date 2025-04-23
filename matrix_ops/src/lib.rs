// pub use matrix::Matrix;
pub use lalgebra_scalar::Scalar;
pub use std::ops::Add;
use std::{fmt::Debug, ops::Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);


impl <T>Add for Matrix<T>  where T:Add<Output = T>+Clone+Copy+Debug{
    type Output = Option<Matrix<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            // panic"Matrix dimensions do not match for addition");
           return  None
        }
        
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] + rhs.0[i][j]);
            }
            result.push(row);
        }
        
        Some(Matrix(result))
    }
}

impl<T:Scalar+Debug+Clone+Copy+Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            // panic!("Matrix dimensions do not match for subtraction");
           return None
        }
        
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] - rhs.0[i][j]);
            }
            result.push(row);
        }
        
        Some(Matrix(result))
    }
}