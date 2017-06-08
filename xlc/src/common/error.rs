use std::fmt;

use super::location;


pub struct Error {
    pub location: Option<location::Location>,
    pub message: String,
}


impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            Some(location) => write!(fmt, "@{}: {}", location, self.message),
            None => write!(fmt, "{}", self.message),
        }
    }
}
