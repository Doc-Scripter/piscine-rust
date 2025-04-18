use std::fmt::Error;
pub use std::fmt::{self, Debug};
pub use std::cmp::{Ord, Ordering};
use std::str::FromStr;


#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}



impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by antigen
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => {
                // If antigens are equal, compare by Rh factor
                self.rh_factor.cmp(&other.rh_factor)
            }
            ordering => ordering,
        }
    }
}

impl FromStr for BloodType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Instead of returning a Result with an Error, we'll panic for invalid inputs
        // This is what the test expects based on the "should panic" annotation
        
        // First, validate the input string
        if s.is_empty() {
            panic!("Empty blood type string");
        }
        
        // Check if the string has a valid format (must end with + or -)
        if !s.ends_with('+') && !s.ends_with('-') {
            panic!("Invalid blood type format: {}", s);
        }
        
        // Extract the antigen part (everything except the last character)
        let antigen_str = &s[0..s.len()-1];
        
        // Validate and convert the antigen part
        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => panic!("Unknown blood type antigen: {}", antigen_str),
        };
        
        // Extract and validate the Rh factor
        let rh_factor = match s.chars().last().unwrap() {
            '+' => RhFactor::Positive,
            '-' => RhFactor::Negative,
            _ => panic!("Invalid Rh factor"), // This should never happen due to the earlier check
        };
        
        Ok(BloodType { antigen, rh_factor })
    }
}



impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh_symbol = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        
        match self.antigen {
            Antigen::A => write!(f, "A{}", rh_symbol),
            Antigen::B => write!(f, "B{}", rh_symbol),
            Antigen::AB => write!(f, "AB{}", rh_symbol),
            Antigen::O => write!(f, "O{}", rh_symbol),
        }
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check Rh factor compatibility
        if matches!(self.rh_factor, RhFactor::Negative) && matches!(other.rh_factor, RhFactor::Positive) {
            return false;
        }

        // Check antigen compatibility
        match other.antigen {
            Antigen::O => true,
            Antigen::A => matches!(self.antigen, Antigen::A | Antigen::AB),
            Antigen::B => matches!(self.antigen, Antigen::B | Antigen::AB),
            Antigen::AB => matches!(self.antigen, Antigen::AB),
        }
    }

	pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();
        
        // Create all possible blood types
        let all_blood_types = [
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];
        
        // Filter for compatible donors
        for blood_type in all_blood_types {
            if self.can_receive_from(&blood_type) {
                donors.push(blood_type);
            }
        }
        
        donors
    }
	
	pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        
        // Create all possible blood types
        let all_blood_types = [
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];
        
        // Filter for compatible recipients
        for blood_type in all_blood_types {
            if blood_type.can_receive_from(self) {
                recipients.push(blood_type);
            }
        }
        
        recipients
    }
}

