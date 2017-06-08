use std::fmt;

use common;
use ast;


pub struct Step<'a> {
    pub node: &'a ast::Node<'a>,
    pub kind: StepKind,
}


pub type Steps<'a> = common::Collection<Step<'a>>;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StepKind {
    Bool,
    Int,
    Nil,
    Str,
    Unk,
}


impl fmt::Display for StepKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StepKind::Bool => write!(fmt, "Boolean"),
            StepKind::Int => write!(fmt, "Integer"),
            StepKind::Nil => write!(fmt, "Nil"),
            StepKind::Str => write!(fmt, "String"),
            StepKind::Unk => write!(fmt, "Unknown"),
        }
    }
}
