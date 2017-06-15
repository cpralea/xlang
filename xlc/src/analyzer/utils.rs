use ast;
use cdata;


pub fn get_identifier_kind<'a>(identifier: &String,
                               steps: &'a cdata::Steps<'a>)
                               -> cdata::StepKind {
    let var = identifier;
    let kind = steps
        .iter()
        .find(|step| match *step.node.kind {
                  ast::NodeKind::Assignment { ref identifier, .. } => {
                      identifier.as_ref().unwrap() == var
                  }
                  _ => false,
              })
        .map(|step| step.kind);
    match kind {
        Some(kind) => kind,
        None => cdata::StepKind::Unk,
    }
}


pub fn get_binary_operands_offsets<'a>(pos: usize, steps: &'a cdata::Steps<'a>) -> (usize, usize) {
    assert!(pos >= 2);
    (skip_expression(pos, 1, steps), 1)
}


fn skip_expression<'a>(pos: usize, i: usize, steps: &'a cdata::Steps<'a>) -> usize {
    match *steps[pos - i].node.kind {
        ast::NodeKind::Expression { ref operator, .. } => {
            match *operator {
                Some(_) => skip_expression(pos, skip_expression(pos, i + 1, steps), steps),
                None => i + 1,
            }
        }
        _ => unreachable!(),
    }
}
