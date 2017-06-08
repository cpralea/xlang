use common;
use ast;
use cdata;


pub fn make_block<'a>(node: &'a ast::Node<'a>,
                      steps: cdata::Steps<'a>)
                      -> common::Status<cdata::Block<'a>> {
    let block = cdata::Block {
        node: node,
        steps: steps,
    };
    common::Status {
        result: block,
        error: None,
    }
}
