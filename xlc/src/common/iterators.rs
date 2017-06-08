pub trait FlexIteratorByRef<'a, T> {
    fn next(&mut self) -> Option<&'a T>;
    fn peek(&self, offset: usize) -> Option<&'a T>;
}


pub trait FlexIteratorByVal<T: Copy> {
    fn next(&mut self) -> Option<T>;
    fn peek(&self, offset: usize) -> Option<T>;
}
