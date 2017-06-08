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
    let (mut l, mut r) = (2, 1);
    assert!(pos >= l && pos >= r);

    while let ast::NodeKind::Expression { ref operator, .. } = *steps[pos - r].node.kind {
        match *operator {
            Some(ref operator) => {
                match operator.as_str() {
                    "+" | "-" | "*" | "/" | "||" | "&&" => {
                        r += 1;
                        l += 2;
                        assert!(pos >= l && pos >= r);
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                r = 1;
                break;
            }
        }
    }

    (l, r)
}
