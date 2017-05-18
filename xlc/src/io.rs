use std::fs;
use std::io;

use common;

use std::io::Read;
use std::io::Write;


pub struct Source {
    data: Vec<char>,
}
impl Source {
    pub fn from_file(file: &String) -> Result<Source, io::Error> {
        let mut file = try!(fs::File::open(file));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        Ok(Source::from_string(content))
    }
    pub fn from_string(content: String) -> Source {
        Source { data: content.chars().collect() }
    }
    pub fn get_line(&self, num: usize) -> Option<String> {
        let (mut index, mut line) = (0, 1);
        while line < num && index < self.data.len() {
            if self.data[index] == '\n' {
                line += 1;
            }
            index += 1;
        }
        if line == num {
            let mut line = String::new();
            while index< self.data.len() && self.data[index] != '\n' {
                line.push(self.data[index]);
                index += 1;
            }
            Some(line)
        } else {
            None
        }
    }
}


pub struct Destination {
    file: fs::File,
}
impl Destination {
    pub fn to_file(file: &String) -> Result<Destination, io::Error> {
        Ok(Destination { file: try!(fs::File::create(file)) })
    }
    pub fn write(&mut self, data: &String) -> Result<(), io::Error> {
        self.file.write_all(data.as_bytes())
            .and(self.file.flush())
    }
}


pub trait SourceFlexIterator: common::FlexIteratorByVal<char> {
    fn location(&self) -> common::SourceLocation;
}
pub struct SourceFlexIter<'a> {
    source: &'a Source,
    index: usize,
    location: common::SourceLocation,
}
impl<'a> SourceFlexIter<'a> {
    fn advance_location(&mut self, chr: &Option<char>) {
        if let Some(chr) = *chr {
            if chr != '\r' {
                if chr == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                } else {
                    self.location.column += 1
                }}}
    }
}
impl<'a> common::FlexIteratorByVal<char> for SourceFlexIter<'a> {
    fn next(&mut self) -> Option<char> {
        let chr = self.peek(0);
        if chr.is_some() {
            self.index += 1;
            self.advance_location(&chr);
        }
        chr
    }
    fn peek(&self, offset: usize) -> Option<char> {
        self.source.data.get(self.index + offset).map(|chr| *chr)
    }
}
impl<'a> SourceFlexIterator for SourceFlexIter<'a> {
    fn location(&self) -> common::SourceLocation {
        self.location
    }
}
impl<'a> Source {
    pub fn iter_flex(&'a self) -> SourceFlexIter<'a> {
        SourceFlexIter {
            source: self,
            index: 0,
            location: common::SourceLocation { line: 1, column: 1 } }
    }
}


macro_rules! log {
    ($tag:expr) => (println!($tag));
    ($tag:expr, $fmt:expr) => (println!(concat!($tag, ": ", $fmt)));
    ($tag:expr, $fmt:expr, $($arg:tt)*) => (println!(concat!($tag, ": ", $fmt), $($arg)*));
}
macro_rules! error {
    () => (log!("[Error]: "));
    ($fmt:expr) => (log!("[Error]", $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!("[Error]", $fmt, $($arg)*));
}
macro_rules! debug {
    () => (log!("[Debug]: "));
    ($fmt:expr) => (log!("[Debug]", $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!("[Debug]", $fmt, $($arg)*));
}
