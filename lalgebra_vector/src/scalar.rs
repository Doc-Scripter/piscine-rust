use std::ops::{Add, Div, Mul, Sub};

/// Trait for scalar values in linear algebra operations.
/// A scalar must support basic arithmetic operations and have identity elements.
pub trait Scalar:Sized+ Add<Output = Self>+ Sub<Output = Self>+ Mul<Output = Self>+ Div<Output = Self>+ PartialEq+ Copy+ Clone
{
    /// Returns the additive identity (zero)
    fn zero() -> Self;

    /// Returns the multiplicative identity (one)
    fn one() -> Self;
}

// Implementation for f64
impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

// Implementation for f32
impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

// Implementation for i32
impl Scalar for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

// Implementation for i64
impl Scalar for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

// Implementation for u32
impl Scalar for u32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

// Implementation for u64
impl Scalar for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}
