#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub mod scalar;
pub use std::fmt::Debug;

pub use scalar::*;

// Implementation for all scalar types
impl<T: Scalar + Debug + Clone + PartialEq> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero(); 1]; 1])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }
}

// Implementation specifically for types that can be created from i32
impl<T: Scalar + Debug + Clone + PartialEq + From<i32>> Matrix<T> {
    pub fn identity(n: usize) -> Self {
        let mut rows = Vec::with_capacity(n);

        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                if i == j {
                    // Diagonal elements are 1
                    row.push(T::from(1));
                } else {
                    // Non-diagonal elements are 0
                    row.push(T::from(0));
                }
            }
            rows.push(row);
        }

        Matrix(rows)
    }
}

// Implementation for u32 and other types that don't implement From<i32>
impl Matrix<u32> {
    pub fn identity_u32(n: usize) -> Self {
        let mut rows = Vec::with_capacity(n);

        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                if i == j {
                    // Diagonal elements are 1
                    row.push(1u32);
                } else {
                    // Non-diagonal elements are 0
                    row.push(0u32);
                }
            }
            rows.push(row);
        }

        Matrix(rows)
    }
}
