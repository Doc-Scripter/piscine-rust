#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

use std::ops::Sub;

impl<T> std::ops::Add for ThreeDVector<T> where  T: std::ops::Add<Output = T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        ThreeDVector{
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl <T>Sub for ThreeDVector<T> where T: Sub<Output = T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        ThreeDVector{
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

