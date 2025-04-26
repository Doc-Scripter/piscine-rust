pub use crate::RomanDigit::*;
/// A single roman digit (including Nulla = 0).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RomanDigit {
    Nulla,
    I, V, X, L, C, D, M,
}

// mapping, in descending order, of values → their roman-digit patterns
const ROMAN_MAP: &[(u32, &[RomanDigit])] = &[
    (1000, &[M]), (900, &[C, M]), (500, &[D]), (400, &[C, D]),
    (100,  &[C]), (90,  &[X, C]), (50,  &[L]), (40,  &[X, L]),
    (10,   &[X]), (9,   &[I, X]), (5,   &[V]), (4,   &[I, V]),
    (1,    &[I]),
];

/// A roman number is just a sequence of RomanDigit
#[derive(Clone, Debug, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![Nulla]);
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

impl RomanNumber {
    /// Decode subtractive‑notation digits back into a u32.
    fn to_int(&self) -> u32 {
        // map each digit to its numeric value
        let vals: Vec<i32> = self.0.iter().map(|d| match d {
            &Nulla => 0,
            &I     => 1,
            &V     => 5,
            &X     => 10,
            &L     => 50,
            &C     => 100,
            &D     => 500,
            &M     => 1000,
        }).collect();

        // accumulate with subtractive logic using signed i32
        let mut total: i32 = 0;
        for i in 0..vals.len() {
            if i + 1 < vals.len() && vals[i] < vals[i+1] {
                total -= vals[i];
            } else {
                total += vals[i];
            }
        }
        // should never be negative
        total.max(0) as u32
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // compute current integer, increment, re-encode
        let curr = self.to_int();
        let next = curr.saturating_add(1);
        let rn = RomanNumber::from(next);
        self.0 = rn.0.clone();
        Some(rn)
    }
}
