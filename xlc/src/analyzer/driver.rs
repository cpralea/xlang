use common;
use ast;
use cdata;

use super::block;
use super::kind;
use super::step;


pub fn analyze<'a>(node: &'a ast::Node<'a>) -> common::Status<cdata::Block<'a>> {
    let mut steps;

    let status = step::build_steps(node);
    steps = status.result;
    if status.error.is_some() {
        return block::make_block(node, steps).error(status.error);
    }

    let status = kind::compute_step_kinds(steps);
    steps = status.result;
    if status.error.is_some() {
        return block::make_block(node, steps).error(status.error);
    }

    block::make_block(node, steps)
}
