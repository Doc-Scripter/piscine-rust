#[derive(Debug,Clone,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub mod scalar;
pub use std::fmt::Debug;

pub use scalar::*;

impl <T: Scalar+Debug+Clone+PartialEq> Matrix<T> {
	pub fn new() -> Matrix<T> {
		Matrix(vec![vec![T::zero(); 1]; 1])
	}
 
	pub fn zero(row: usize, col: usize) -> Matrix<T> {
		Matrix(vec![vec![T::zero();col];row])
	}

	pub fn identity(n: usize) -> Self {
        let mut rows = Vec::with_capacity(n);
        
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                if i == j {
                    // Diagonal elements are 1
                    row.push(T::one());
                } else {
                    // Non-diagonal elements are 0
                    row.push(T::zero());
                }
            }
            rows.push(row);
        }
        
        Matrix(rows)
    }
	
}