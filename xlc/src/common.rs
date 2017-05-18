use std::fmt;
use std::iter;
use std::ops;


pub const TAB: &str = "  ";
pub const NL: &str = "\n";


#[derive(Copy, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}
impl fmt::Display for SourceLocation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{},{}", self.line, self.column)
    }
}


pub struct Error {
    pub location: Option<SourceLocation>,
    pub message: String,
}
impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            Some(location)  => { write!(fmt, "@{}: {}", location, self.message) }
            None            => { write!(fmt, "{}", self.message) }
        }
    }
}


pub struct Status<T> {
    pub result: T,
    pub error: Option<Error>,
}
impl<T> Status<T> {
    pub fn error(mut self, error: Option<Error>) -> Self {
        self.error = error;
        self
    }
}


pub trait FlexIteratorByRef<'a, T> {
    fn next(&mut self) -> Option<&'a T>;
    fn peek(&self, offset: usize) -> Option<&'a T>;
}
pub trait FlexIteratorByVal<T> where T: Copy {
    fn next(&mut self) -> Option<T>;
    fn peek(&self, offset: usize) -> Option<T>;
}


pub struct Collection<T> {
    items: Vec<T>,
}
impl<T> Collection<T> {
    pub fn new() -> Collection<T> {
        Collection { items: Vec::new() }
    }
    pub fn iter_flex<'a>(&'a self) -> CollectionFlexIter<'a, T> {
        CollectionFlexIter { collection: self, index: 0 }
    }
}


pub struct CollectionFlexIter<'a, T: 'a> {
    collection: &'a Collection<T>,
    index: usize,
}
impl<'a, T> FlexIteratorByRef<'a, T> for CollectionFlexIter<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        let item = self.collection.items.get(self.index);
        if item.is_some() {
            self.index += 1;
        }
        item
    }
    fn peek(&self, offset: usize) -> Option<&'a T> {
        self.collection.items.get(self.index + offset)
    }
}


impl<T> ops::Index<usize> for Collection<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.items.index(index)
    }
}
impl<T> ops::IndexMut<usize> for Collection<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.items.index_mut(index)
    }
}
impl<T> ops::Deref for Collection<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.items.deref()
    }
}
impl<T> Collection<T> {
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}


pub fn take(times: usize, string: &str) -> String {
    iter::repeat(string).take(times).collect::<String>()
}
