use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Flag {
            short_hand: format!("-{}", name.to_string().chars().next().unwrap().to_string()),
            long_hand: format!(
                "--{}",
                name.to_string()
            ),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand.clone(), func);
        self.flags.insert(flag.long_hand.clone(), func);
    }
    /*
    exec_func, which executes the function using the flag provided and returns the result. The callback should be executed with the first two arguments of the supplied argv argument. Return either the successful result from the callback or the error stringified.
    */
    #[allow(unused_variables)]
    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let value = self.flags.get(input);

        match value {
            Some(v) => {
            
                match v(argv[0], argv[1]) {
                    Ok(v) => return Ok(v),
                    Err(err) => return Err("invalid float literal".to_owned()),
                };
            }
            None =>  return Err("invalid float literal".to_owned())
        }
        

        // Ok("invalid float literal".to_owned())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_parsed = a.parse::<f64>();
    let b_parsed = b.parse::<f64>();

    let a = a_parsed?;
    let b = b_parsed?;

    //  if b == 0.0 {
    //      return "invalid".parse::<f64>().map(|_| unreachable!());
    //  }

    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_parsed = a.parse::<f64>();
    let b_parsed = b.parse::<f64>();

    let a = a_parsed?;
    let b = b_parsed?;

    // if b == 0.0 {
    //    return Ok("inf".parse::<f64>().map(|_| unreachable!())?);
    // }

    Ok((a % b).to_string())
}
