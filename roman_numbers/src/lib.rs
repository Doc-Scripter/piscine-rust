pub use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::M,
            500 => RomanDigit::D,
            1000 => RomanDigit::C,
            0 => RomanDigit::Nulla,
            _=> panic!("Invalid value for RomanDigit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        let mut val = value as i32;
        let mut vec: Vec<RomanDigit> = Vec::new();
        if val==0{
            vec.push(RomanDigit::Nulla);
            return RomanNumber(vec);
        }
        while val > 0 {
            match val {
                v if v >= 1000 => {
                    vec.push(RomanDigit::M);
                    val -= 1000;
                }
                v if v >= 900 => {
                    vec.push(RomanDigit::M);
                    vec.push(RomanDigit::C);
                    val -= 900;
                }
                v if v >= 500 => {
                    vec.push(RomanDigit::D);
                    val -= 500;
                }
                v if v >= 400 => {
                    vec.push(RomanDigit::C);
                    vec.push(RomanDigit::D);
                    val -= 400;
                }
                v if v >= 100 => {
                    vec.push(RomanDigit::M);
                    val -= 100;
                }
                v if v >= 90 => {
                    vec.push(RomanDigit::X);
                    vec.push(RomanDigit::M);
                    val -= 90;
                }
                v if v >= 50 => {
                    vec.push(RomanDigit::L);
                    val -= 50;
                }
                v if v >= 40 => {
                    vec.push(RomanDigit::X);
                    vec.push(RomanDigit::L);
                    val -= 40;
                }
                v if v >= 10 => {
                    vec.push(RomanDigit::X);
                    val -= 10;
                }
                v if v >= 9 => {
                    vec.push(RomanDigit::I);
                    vec.push(RomanDigit::X);
                    val -= 9;
                }
                v if v >= 5 => {
                    vec.push(RomanDigit::V);
                    val -= 5;
                }
                v if v >= 4 => {
                    vec.push(RomanDigit::I);
                    vec.push(RomanDigit::V);
                    val -= 4;
                }
                v if v >= 1 => {
                    vec.push(RomanDigit::I);
                    val -= 1;
                }
                _ => panic!("Invalid value")
            }
        }
        RomanNumber(vec)
    }
}
