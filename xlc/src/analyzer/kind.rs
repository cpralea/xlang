use common;
use ast;
use cdata;

use super::assignment;
use super::expression;


pub fn compute_step_kinds<'a>(mut steps: cdata::Steps<'a>) -> common::Status<cdata::Steps<'a>> {
    for pos in 0..steps.len() {
        let status = match *steps[pos].node.kind {
            ast::NodeKind::Expression { .. } => {
                expression::compute_expression_step_kind(steps, pos)
            }
            ast::NodeKind::Assignment { .. } => {
                assignment::compute_assignment_step_kind(steps, pos)
            }
            _ => assign_nil_step_kind(steps, pos),
        };
        steps = status.result;
        if status.error.is_some() {
            return common::Status {
                result: steps,
                error: status.error,
            };
        }
    }

    common::Status {
        result: steps,
        error: None,
    }
}


fn assign_nil_step_kind<'a>(
    mut steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
    steps[pos].kind = cdata::StepKind::Nil;
    common::Status {
        result: steps,
        error: None,
    }
}
