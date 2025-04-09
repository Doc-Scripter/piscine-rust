pub use std::{error::Error, fmt::{self, Display}};

#[derive(Debug)]
pub enum ParseErr {
    pub Empty,
    pub Malformed(Box<dyn Error>),
}

impl Display for ParseErr {
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

impl Error for ParseErr {
    pub fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}

impl Error for ReadErr {
    pub fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}
