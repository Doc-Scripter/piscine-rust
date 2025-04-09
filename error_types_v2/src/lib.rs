// this will be the structure that wil handle the errors

use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, pass: String) -> Form {
        Form {
            name,
            password: pass,
        }
    }
    #[allow(unused)]
    pub fn validate(&self) -> Result<(), FormError> {
        // let mut erro_map = HashMap::new();
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "name",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }
        let mut digits = false;
        let mut alpha_numeric = false;
        let mut alphabet: bool = false;
        let mut checks = [&digits, &alpha_numeric, &alphabet];
        for val in self.password.chars() {
            if val.is_ascii_alphabetic() {
                checks[2] = &true;
            }
            if val.is_ascii_digit() {
                checks[0] = &true;
            }
            if val.is_ascii_punctuation() {
                checks[1] = &true;
            }
        }

        if checks.iter().all(|e| **e == true) {
            return Ok(());
        } else {
            return Err(FormError::new(
                "password",
                self.password.to_owned(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }
    }
}

// type Error;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
#[allow(unused)]
impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        // let mut form_values: HashMap<String, String> = HashMap::new();
        // form_values.insert(field_name.to_owned(), field_value);
        let dt = Utc.with_ymd_and_hms(2022, 10, 17, 12, 9, 25).unwrap();
        FormError {
            form_values: (field_name.to_owned(), field_value),
            date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_owned(),
        }
    }
}

//2022-10-17 12:09:25
