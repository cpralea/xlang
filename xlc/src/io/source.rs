use std::fs;
use std::io;

use common;

use std::io::Read;


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

    pub fn iter_flex<'a>(&'a self) -> SourceFlexIter<'a> {
        SourceFlexIter {
            source: self,
            index: 0,
            location: common::Location { line: 1, column: 1 },
        }
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
            while index < self.data.len() && self.data[index] != '\n' {
                line.push(self.data[index]);
                index += 1;
            }
            Some(line)
        } else {
            None
        }
    }
}


pub trait SourceFlexIterator: common::FlexIteratorByVal<char> {
    fn location(&self) -> common::Location;
}


pub struct SourceFlexIter<'a> {
    source: &'a Source,
    index: usize,
    location: common::Location,
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
                }
            }
        }
    }
}


impl<'a> SourceFlexIterator for SourceFlexIter<'a> {
    fn location(&self) -> common::Location {
        self.location
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
