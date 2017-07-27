use std::ops;

use super::iterators;


pub struct Collection<T> {
    items: Vec<T>,
}


impl<T> Collection<T> {
    pub fn new() -> Collection<T> {
        Collection { items: Vec::new() }
    }
    pub fn iter_flex<'a>(&'a self) -> CollectionFlexIter<'a, T> {
        CollectionFlexIter {
            collection: self,
            index: 0,
        }
    }
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


pub struct CollectionFlexIter<'a, T: 'a> {
    collection: &'a Collection<T>,
    index: usize,
}


impl<'a, T> iterators::FlexIteratorByRef<'a, T> for CollectionFlexIter<'a, T> {
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


impl<'a, T: Copy> iterators::FlexIteratorByVal<T> for CollectionFlexIter<'a, T> {
    fn next(&mut self) -> Option<T> {
        (self as &mut iterators::FlexIteratorByRef<'a, T>).next().map(|e| *e)
    }
    fn peek(&self, offset: usize) -> Option<T> {
        (self as &iterators::FlexIteratorByRef<'a, T>).peek(offset).map(|e| *e)
    }
}
