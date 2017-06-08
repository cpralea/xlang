use super::error;


pub struct Status<T> {
    pub result: T,
    pub error: Option<error::Error>,
}


impl<T> Status<T> {
    pub fn error(mut self, error: Option<error::Error>) -> Self {
        self.error = error;
        self
    }
}
