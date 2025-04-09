// this will be the structure that wil handle the errors

// use chrono::TimeZone;
// use chrono::offset::Utc;

use chrono::TimeZone;

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
                "name".to_string(),
                "".to_string(),
                "Username is empty".to_string(),
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Password should be at least 8 characters long".to_string(),
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
                "password".to_string(),
                self.password.to_owned(),
                "Password should be a combination of ASCII numbers, letters and symbols".to_string(),
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
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> Self {
        // let mut form_values: HashMap<String, String> = HashMap::new();
        // form_values.insert(field_name.to_owned(), field_value);
        let dt = chrono::offset::Utc.with_ymd_and_hms(2022, 10, 17, 12, 9, 25).unwrap();
        FormError {
            form_values: (field_name , field_value),
            date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

//2022-10-17 12:09:25

/*

   Compiling autocfg v1.4.0
   Compiling iana-time-zone v0.1.62
   Compiling num-traits v0.2.19
   Compiling chrono v0.4.40
   Compiling error_types v0.1.0 (/jail/solutions/error_types)
   Compiling error_types_test v0.1.0 (/jail/tests/error_types_test)
warning: unused import: `error_types::*`
 --> src/lib.rs:1:5
  |
1 | use error_types::*;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `error_types_test` (lib) generated 1 warning (run `cargo fix --lib -p error_types_test` to apply 1 suggestion)
error[E0308]: mismatched types
  --> src/lib.rs:19:31
   |
19 |                 form_values: ("name", "".to_owned()),
   |                               ^^^^^^- help: try using a conversion method: `.to_string()`
   |                               |
   |                               expected `String`, found `&str`

error[E0433]: failed to resolve: use of undeclared type `Utc`
  --> src/lib.rs:20:23
   |
20 |                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
   |                       ^^^ use of undeclared type `Utc`

error[E0308]: mismatched types
  --> src/lib.rs:21:22
   |
21 |                 err: "Username is empty",
   |                      ^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                      |
   |                      expected `String`, found `&str`

error[E0308]: mismatched types
  --> src/lib.rs:30:31
   |
30 |                 form_values: ("password", "12345".to_owned()),
   |                               ^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                               |
   |                               expected `String`, found `&str`

error[E0433]: failed to resolve: use of undeclared type `Utc`
  --> src/lib.rs:31:23
   |
31 |                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
   |                       ^^^ use of undeclared type `Utc`

error[E0308]: mismatched types
  --> src/lib.rs:32:22
   |
32 |                 err: "Password should be at least 8 characters long",
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                      |
   |                      expected `String`, found `&str`

error[E0308]: mismatched types
  --> src/lib.rs:41:31
   |
41 |                 form_values: ("password", "sdASDsrW".to_owned()),
   |                               ^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                               |
   |                               expected `String`, found `&str`

error[E0433]: failed to resolve: use of undeclared type `Utc`
  --> src/lib.rs:42:23
   |
42 |                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
   |                       ^^^ use of undeclared type `Utc`

error[E0308]: mismatched types
  --> src/lib.rs:43:22
   |
43 |                 err: "Password should be a combination of ASCII numbers, letters and symbols",
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                      |
   |                      expected `String`, found `&str`

error[E0308]: mismatched types
  --> src/lib.rs:52:31
   |
52 |                 form_values: ("password", "dsGE1SAD213".to_owned()),
   |                               ^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                               |
   |                               expected `String`, found `&str`

error[E0433]: failed to resolve: use of undeclared type `Utc`
  --> src/lib.rs:53:23
   |
53 |                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
   |                       ^^^ use of undeclared type `Utc`

error[E0308]: mismatched types
  --> src/lib.rs:54:22
   |
54 |                 err: "Password should be a combination of ASCII numbers, letters and symbols",
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                      |
   |                      expected `String`, found `&str`

error[E0308]: mismatched types
  --> src/lib.rs:63:31
   |
63 |                 form_values: ("password", String::from("dsaSD&%DF!?=")),
   |                               ^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                               |
   |                               expected `String`, found `&str`

error[E0433]: failed to resolve: use of undeclared type `Utc`
  --> src/lib.rs:64:23
   |
64 |                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
   |                       ^^^ use of undeclared type `Utc`

error[E0308]: mismatched types
  --> src/lib.rs:65:22
   |
65 |                 err: "Password should be a combination of ASCII numbers, letters and symbols",
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                      |
   |                      expected `String`, found `&str`

Some errors have detailed explanations: E0308, E0433.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `error_types_test` (lib test) due to 15 previous errors
*/