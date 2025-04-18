use std::ops::*;
use std::fmt::Debug;

pub trait Scalar: Add<Output = Self> + std::ops::Mul<Output = Self> + Copy + Debug {
    fn zero() -> Self;
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
}
impl Scalar for i64 {
    fn zero() -> Self { 0 }
}

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            panic!("Cannot add vectors of different lengths");
        }
        
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            result.push(self.0[i] + other.0[i]);
        }
        
        Vector(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Calculates the dot product between two vectors
    /// Returns None if the vectors have different lengths
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let mut result = T::zero();
        for i in 0..self.0.len() {
            result = result + (self.0[i] * other.0[i]);
        }
        
        Some(result)
    }

    /// Calculates the vector product between two vectors
    /// Returns None if the vectors have different lengths
    pub fn vector_product(&self, other: &Self) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            result.push(self.0[i] * other.0[i]);
        }
        
        Some(Vector(result))
    }
}
