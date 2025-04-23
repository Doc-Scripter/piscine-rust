// todo: lalgebra_vector
/* 
Instructions
A vector in linear algebra is defined as "anything that can be added, and that can be multiplied by a scalar".

Define the associated function dot, that calculates the dot product between two vectors. If the vectors are of different lengths, return None.

Note: Vector must implement Debug, Clone, Eq and PartialEq.

Expected Functions and Structure
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

impl Add for Vector<T> {
}

impl Vector<T> {
	pub fn new() -> Self {
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
	}
} */
// pub use lalgebra_scalar::Scalar;
pub mod scalar;
pub use scalar::Scalar;
pub use std::ops::Add;
pub use std::fmt::Debug;
pub use std::clone::Clone;
pub use std::cmp::{Eq, PartialEq};
pub struct Vector<T: Scalar + Debug + Clone + Eq + PartialEq>(pub Vec<T>);

impl<T: Scalar + Debug + Clone + Eq + PartialEq + Add<Output = T>> Add for Vector<T> {
    type Output = Option<Vector<T>>;
    
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let sum: Vec<T> = self.0.iter().zip(other.0.iter()).map(|(a, b)| a.clone() + b.clone()).collect();
        Some(Vector(sum))
    }
}

impl<T: Scalar + Debug + Clone + Eq + PartialEq> Vector<T> {

    pub fn new(vec: Vec<T>) -> Self {
        Self(vec)
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let dot_product = self.0.iter().zip(other.0.iter()).map(|(a, b)| a.clone() * b.clone()).fold(T::zero(), |acc, x| acc + x);
        Some(dot_product)
    }
}

impl<T: Scalar + Debug + Clone + Eq + PartialEq> Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({:?})", self.0)
    }
}

impl<T: Scalar + Debug + Clone + Eq + PartialEq> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector(self.0.clone())
    }
}

impl<T: Scalar + Debug + Clone + Eq + PartialEq> Eq for Vector<T> {}

impl<T: Scalar + Debug + Clone + Eq + PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}