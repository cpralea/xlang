use common;
use ast;


pub struct CompilerData<'a> {
    pub node: &'a ast::Node<'a>,
    pub steps: Timeline<Step<'a>>,
}


pub struct Step<'a> {
    pub node: &'a ast::Node<'a>,
    pub kind: StepKind,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StepKind {
    Int,
    Nil,
    Str,
    Unk,
}


pub type Timeline<T> = common::Collection<T>;
