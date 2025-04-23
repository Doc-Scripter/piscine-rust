pub use std::ops::AddAssign;
pub use std::ops::Mul;
pub use lalgebra_scalar::*;

#[derive(Clone,Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T:Clone>Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
        self.0[0].iter().count() as usize

    }
    
	pub fn number_of_rows(&self) -> usize {
        self.0.iter().count() as usize
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        for i in 0..self.0.len() {
            if i == n {
                return self.0[i].clone();
            }
        }
        panic!("Row not found");
	}

	pub fn col(&self, n: usize) -> Vec<T> {
        let mut col = Vec::new();
        for i in 0..self.0.len() {
            col.push(self.0[i][n].clone());
        }
        col
	}
}

impl<T:Mul<Output = T>+Scalar+AddAssign> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..rhs.0[0].len() {
                let mut sum = T::zero();
                for k in 0..self.0[0].len() {
                    sum += self.0[i][k] * rhs.0[k][j];
                }
                row.push(sum);
            }
            result.push(row);
        }
        Some(Matrix(result))

    }
}