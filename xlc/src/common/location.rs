use std::fmt;


#[derive(Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}


impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{},{}", self.line, self.column)
    }
}
