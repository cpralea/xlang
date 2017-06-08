use ref_eq::ref_eq;

use common;
use ast;
use cdata;

use super::utils;


pub fn compute_assignment_step_kind<'a>(mut steps: cdata::Steps<'a>,
                                        pos: usize)
                                        -> common::Status<cdata::Steps<'a>> {
    let mut error = None;

    match *steps[pos].node.kind {
        ast::NodeKind::Assignment {
            ref identifier,
            ref expression,
        } => {
            assert!(pos != 0 && identifier.is_some());
            assert!(ref_eq(steps[pos - 1].node, expression.as_ref().unwrap()));
            assert!(steps[pos - 1].kind != cdata::StepKind::Unk);
            let identifier = identifier.as_ref().unwrap();
            let (old, new) = (utils::get_identifier_kind(identifier, &steps), steps[pos - 1].kind);
            if old == cdata::StepKind::Unk || new == old {
                steps[pos].kind = steps[pos - 1].kind;
            } else {
                error = Some(common::Error {
                                 location: Some(steps[pos].node.token.unwrap().location),
                                 message: format!("Cannot assign {} to {}.", new, old),
                             });
            }
        }
        _ => unreachable!(),
    }

    common::Status {
        result: steps,
        error: error,
    }
}
