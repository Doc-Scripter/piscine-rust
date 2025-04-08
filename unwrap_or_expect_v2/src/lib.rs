pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}


pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(url) => {
            match security_level {
                Security::Unknown => url.to_owned().to_string(),
                Security::Message => url.to_owned().to_string(),
                Security::Warning => url.to_owned().to_string(),
                Security::NotFound => url.to_owned().to_string(),
                Security::UnexpectedUrl => panic!("malicious_server.com"),
            }
        }
        Err(url) => {
            match security_level {
                Security::Unknown => panic!("called `Result::unwrap()` on an `Err` value: \"{}\"", url),
                Security::Message => panic!("ERROR: program stops"),
                Security::Warning => format!("WARNING: check the server"),
                Security::NotFound => format!("Not found: {}",url.to_owned()),
                Security::UnexpectedUrl => panic!("{}",url),
            }
        }
    }
}
