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
        // Check for empty string
        if s.is_empty() {
            panic!("Empty blood type string");
        }

        // Parse the antigen part
        let antigen = if s.starts_with("AB") {
            Antigen::AB
        } else if s.starts_with('A') {
            Antigen::A
        } else if s.starts_with('B') {
            Antigen::B
        } else if s.starts_with('O') {
            Antigen::O
        } else {
            panic!("Invalid blood type antigen: {}", s);
        };

        // Parse the Rh factor
        let rh_factor = if s.ends_with('+') {
            RhFactor::Positive
        } else if s.ends_with('-') {
            RhFactor::Negative
        } else {
            panic!("Invalid Rh factor in blood type: {}", s);
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

