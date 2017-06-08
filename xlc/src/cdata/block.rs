use ast;

use super::step;


pub struct Block<'a> {
    pub node: &'a ast::Node<'a>,
    pub steps: step::Steps<'a>,
}
