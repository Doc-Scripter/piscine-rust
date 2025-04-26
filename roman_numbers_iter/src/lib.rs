
pub use crate::RomanDigit::*;
/// A single roman digit (including Nulla = 0).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RomanDigit {
    Nulla,
    I, V, X, L, C, D, M,
}
// Define the four “powers” and subtractive composites in descending order
const ROMAN_MAP: &[(u32, &[RomanDigit])] = &[
    (1000, &[M]),
    (900,  &[C, M]),
    (500,  &[D]),
    (400,  &[C, D]),
    (100,  &[C]),
    (90,   &[X, C]),
    (50,   &[L]),
    (40,   &[X, L]),
    (10,   &[X]),
    (9,    &[I, X]),
    (5,    &[V]),
    (4,    &[I, V]),
    (1,    &[I]),
];

/// A roman number is just a sequence of RomanDigit
#[derive(Clone, Debug, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        // special case zero
        if n == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }
        let mut digits = Vec::new();
        for &(value, pattern) in ROMAN_MAP {
            while n >= value {
                n -= value;
                digits.extend_from_slice(pattern);
            }
        }
        RomanNumber(digits)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanDigit;

    /// Yield one digit at a time, front-to-back.
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            // remove at index 0 to yield in stored order
            Some(self.0.remove(0))
        }
    }
}

// -- helper function for direct step‐counting tests (not strictly needed) --

/// Compute the number of Collatz steps for n
pub fn collatz(n: u64) -> usize {
    if n < 2 { return 0; }
    let mut count = 0;
    let mut v = n;
    while v != 1 {
        v = if v % 2 == 0 { v/2 } else { 3*v + 1 };
        count += 1;
    }
    count
}


