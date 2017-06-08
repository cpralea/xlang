use std::fs;
use std::io;

use std::io::Write;


pub struct Destination {
    file: fs::File,
}


impl Destination {
    pub fn to_file(file: &String) -> Result<Destination, io::Error> {
        Ok(Destination { file: try!(fs::File::create(file)) })
    }

    pub fn write(&mut self, data: &String) -> Result<(), io::Error> {
        self.file.write_all(data.as_bytes()).and(self.file.flush())
    }
}
