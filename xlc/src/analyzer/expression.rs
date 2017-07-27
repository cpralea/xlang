use common;
use ast;
use cdata;

use super::utils;


pub fn compute_expression_step_kind<'a>(
    steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
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
                return compute_for_boolean(boolean.as_ref().unwrap(), steps, pos);
            }
            if identifier.is_some() {
                return compute_for_identifier(identifier.as_ref().unwrap(), steps, pos);
            }
            if integer.is_some() {
                return compute_for_integer(integer.as_ref().unwrap(), steps, pos);
            }
            if string.is_some() {
                return compute_for_string(string.as_ref().unwrap(), steps, pos);
            }
            if operator.is_some() {
                return compute_for_operator(operator.as_ref().unwrap(), steps, pos);
            }
            unreachable!();
        }
        _ => unreachable!(),
    }
}


pub fn compute_for_operator<'a>(
    operator: &String,
    mut steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
    let mut error = None;

    let (l, r) = utils::get_binary_operands_offsets(pos, &steps);
    if let Some(kind) = match operator.as_str() {
        "+" | "-" | "*" | "/" | "||" | "&&" | "!" => {
            if steps[pos - l].kind == cdata::StepKind::Bool &&
                steps[pos - r].kind == cdata::StepKind::Bool
            {
                Some(cdata::StepKind::Bool)
            } else if steps[pos - l].kind == cdata::StepKind::Int &&
                       steps[pos - r].kind == cdata::StepKind::Int
            {
                Some(cdata::StepKind::Int)
            } else {
                None
            }
        }
        "==" | "!=" => {
            if steps[pos - l].kind == cdata::StepKind::Bool &&
                steps[pos - r].kind == cdata::StepKind::Bool ||
                steps[pos - l].kind == cdata::StepKind::Int &&
                    steps[pos - r].kind == cdata::StepKind::Int
            {
                Some(cdata::StepKind::Bool)
            } else {
                None
            }
        }
        "<" | "<=" | ">" | ">=" => {
            if steps[pos - l].kind == cdata::StepKind::Int &&
                steps[pos - r].kind == cdata::StepKind::Int
            {
                Some(cdata::StepKind::Bool)
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
    {
        steps[pos].kind = kind;
    } else {
        let message = match steps[pos - l].node.token.is_some() {
            true => {
                format!(
                    "Cannot apply binary '{}' to {} and {}.",
                    operator,
                    steps[pos - l].kind,
                    steps[pos - r].kind
                )
            }
            false => format!("Cannot apply unary '{}' to {}.", operator, steps[pos - r].kind),
        };
        error = Some(common::Error {
            location: Some(steps[pos].node.token.unwrap().location),
            message: message,
        })
    }

    common::Status {
        result: steps,
        error: error,
    }
}


pub fn compute_for_identifier<'a>(
    identifier: &String,
    mut steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
    let mut error = None;

    steps[pos].kind = utils::get_identifier_kind(identifier, &steps);
    if steps[pos].kind == cdata::StepKind::Unk {
        error = Some(common::Error {
            location: Some(steps[pos].node.token.unwrap().location),
            message: format!("Unknown identifier '{}'.", identifier),
        });
    }

    common::Status {
        result: steps,
        error: error,
    }
}


pub fn compute_for_boolean<'a>(
    _boolean: &bool,
    mut steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
    steps[pos].kind = cdata::StepKind::Bool;

    common::Status {
        result: steps,
        error: None,
    }
}


pub fn compute_for_integer<'a>(
    _integer: &i64,
    mut steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
    steps[pos].kind = cdata::StepKind::Int;

    common::Status {
        result: steps,
        error: None,
    }
}


pub fn compute_for_string<'a>(
    _string: &String,
    mut steps: cdata::Steps<'a>,
    pos: usize,
) -> common::Status<cdata::Steps<'a>> {
    steps[pos].kind = cdata::StepKind::Str;

    common::Status {
        result: steps,
        error: None,
    }
}
