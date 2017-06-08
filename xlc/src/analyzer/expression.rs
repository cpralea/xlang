use common;
use ast;
use cdata;

use super::utils;


pub fn compute_expression_step_kind<'a>(mut steps: cdata::Steps<'a>,
                                        pos: usize)
                                        -> common::Status<cdata::Steps<'a>> {
    let mut error = None;

    match *steps[pos].node.kind {
        ast::NodeKind::Expression {
            ref boolean,
            ref identifier,
            ref integer,
            ref string,
            ref operator,
            ..
        } => {
            if boolean.is_some() {
                steps[pos].kind = cdata::StepKind::Bool;
            }
            if identifier.is_some() {
                let identifier = identifier.as_ref().unwrap();
                steps[pos].kind = utils::get_identifier_kind(identifier, &steps);
                if steps[pos].kind == cdata::StepKind::Unk {
                    error = Some(common::Error {
                                     location: Some(steps[pos].node.token.unwrap().location),
                                     message: format!("Unknown identifier '{}'.", identifier),
                                 });
                }
            }
            if integer.is_some() {
                steps[pos].kind = cdata::StepKind::Int;
            }
            if string.is_some() {
                steps[pos].kind = cdata::StepKind::Str;
            }
            if operator.is_some() {
                match operator.as_ref().unwrap().as_str() {
                    "+" | "-" | "*" | "/" | "||" | "&&" => {
                        let (l, r) = utils::get_binary_operands_offsets(pos, &steps);
                        if steps[pos - l].kind == cdata::StepKind::Bool &&
                           steps[pos - r].kind == cdata::StepKind::Bool {
                            steps[pos].kind = cdata::StepKind::Bool;
                        } else if steps[pos - l].kind == cdata::StepKind::Int &&
                                  steps[pos - r].kind == cdata::StepKind::Int {
                            steps[pos].kind = cdata::StepKind::Int;
                        } else {
                            error = Some(common::Error {
                                             location:
                                                 Some(steps[pos].node.token.unwrap().location),
                                             message: format!("Cannot apply '{}' to {} and {}.",
                                                              operator.as_ref().unwrap(),
                                                              steps[pos - l].kind,
                                                              steps[pos - r].kind),
                                         });
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }

    common::Status {
        result: steps,
        error: error,
    }
}
