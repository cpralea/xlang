use common;
use ast;
use cdata;

use ast::NodeVisitor;


pub fn build_steps<'a>(node: &'a ast::Node<'a>) -> common::Status<cdata::Steps<'a>> {
    let mut builder = StepsBuilder::new();
    node.accept(&mut builder);
    builder.status()
}


struct StepsBuilder<'a> {
    skip: bool,
    status: common::Status<cdata::Steps<'a>>,
}


impl<'a> StepsBuilder<'a> {
    fn new() -> StepsBuilder<'a> {
        StepsBuilder {
            skip: false,
            status: common::Status {
                result: cdata::Steps::new(),
                error: None,
            },
        }
    }
}


impl<'a> ast::NodeVisitor<'a> for StepsBuilder<'a> {
    type Result = cdata::Steps<'a>;

    fn end_visit_node(&mut self, node: &'a ast::Node<'a>) {
        if !self.skip {
            let step = cdata::Step {
                node: node,
                kind: cdata::StepKind::Unk,
            };
            self.status.result.push(step);
        }
        self.skip = false;
    }
    fn end_visit_program(&mut self, _program: &'a ast::Node<'a>) {
        self.skip = true;
    }
    fn status(self) -> common::Status<Self::Result> {
        self.status
    }
}
