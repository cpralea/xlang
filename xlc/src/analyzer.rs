use ref_eq::ref_eq;

use common;
use ast;
use cdata;

use ast::NodeVisitor;


pub fn analyze<'a>(
           node: &'a ast::Node<'a>)
        -> common::Status<cdata::CompilerData<'a>> {

    let mut steps;

    let status = build_steps(node);
    steps = status.result;
    if status.error.is_some() {
        return make_cdata(node, steps).error(status.error);
    }

    let status = compute_step_kinds(steps);
    steps = status.result;
    if status.error.is_some() {
        return make_cdata(node, steps).error(status.error);
    }

    make_cdata(node, steps)
}


fn build_steps<'a>(
           node: &'a ast::Node<'a>)
        -> common::Status<cdata::Timeline<cdata::Step<'a>>> {

    let mut builder = StepsBuilder::new();
    node.accept(&mut builder);
    builder.status()
}


fn compute_step_kinds<'a>(
           mut steps: cdata::Timeline<cdata::Step<'a>>)
        -> common::Status<cdata::Timeline<cdata::Step<'a>>> {

    for pos in 0..steps.len() {
        let status = match *steps[pos].node.kind {
            ast::NodeKind::Expression {..} => compute_expression_step_kind(steps, pos),
            ast::NodeKind::Assignment {..} => compute_assignment_step_kind(steps, pos),
            _ => assign_nil_step_kind(steps, pos),
        };
        steps = status.result;
        if status.error.is_some() {
            return common::Status { result: steps, error: status.error };
        }
    }

    common::Status { result: steps, error: None }
}


fn compute_expression_step_kind<'a>(
           mut steps: cdata::Timeline<cdata::Step<'a>>,
           pos: usize)
        -> common::Status<cdata::Timeline<cdata::Step<'a>>> {

    let mut error = None;

    match *steps[pos].node.kind {
        ast::NodeKind::Expression { ref identifier, ref integer, ref string } => {
            if identifier.is_some() {
                let identifier = identifier.as_ref().unwrap();
                steps[pos].kind = get_identifier_kind(identifier, &steps);
                if steps[pos].kind == cdata::StepKind::Unk {
                    error = Some(common::Error {
                        location: Some(steps[pos].node.token.unwrap().location),
                        message: format!("Unknown identifier '{}'.", identifier) });
                }
            }
            if integer.is_some() {
                steps[pos].kind = cdata::StepKind::Int;
            }
            if string.is_some() {
                steps[pos].kind = cdata::StepKind::Str;
            }
        }, _ => unreachable!(), }

    common::Status { result: steps, error: error }
}


fn compute_assignment_step_kind<'a>(
           mut steps: cdata::Timeline<cdata::Step<'a>>,
           pos: usize)
        -> common::Status<cdata::Timeline<cdata::Step<'a>>> {

    let mut error = None;

    match *steps[pos].node.kind {
        ast::NodeKind::Assignment { ref identifier, ref expression } => {
            assert!(pos != 0 && identifier.is_some());
            assert!(ref_eq(steps[pos - 1].node, expression.as_ref().unwrap()));
            assert!(steps[pos - 1].kind != cdata::StepKind::Unk);
            let identifier = identifier.as_ref().unwrap();
            let (old, new) = (get_identifier_kind(identifier, &steps), steps[pos - 1].kind);
            if old == cdata::StepKind::Unk || new == old {
                steps[pos].kind = steps[pos - 1].kind;
            } else {
                error = Some(common::Error {
                    location: Some(steps[pos].node.token.unwrap().location),
                    message: format!("Cannot assign {:?} to {:?}.", new, old) });
            }
        }, _ => unreachable!(), }

    common::Status { result: steps, error: error }
}


fn assign_nil_step_kind<'a>(
           mut steps: cdata::Timeline<cdata::Step<'a>>,
           pos: usize)
        -> common::Status<cdata::Timeline<cdata::Step<'a>>> {

    steps[pos].kind = cdata::StepKind::Nil;
    common::Status { result: steps, error: None }
}


fn get_identifier_kind<'a>(
           identifier: &String,
           steps: &'a cdata::Timeline<cdata::Step<'a>>)
        -> cdata::StepKind {

    let var = identifier;
    let kind = steps.iter()
        .find(|step| {
            match *step.node.kind {
                ast::NodeKind::Assignment { ref identifier, .. } =>
                    identifier.as_ref().unwrap() == var,
                _ => false, }})
        .map(|step| step.kind);
    match kind {
        Some(kind) => kind,
        None       => cdata::StepKind::Unk,
    }
}


fn make_cdata<'a>(
           node: &'a ast::Node<'a>,
           steps: cdata::Timeline<cdata::Step<'a>>)
        -> common::Status<cdata::CompilerData<'a>> {

    let cdata = cdata::CompilerData { node: node, steps: steps };
    common::Status { result: cdata, error: None }
}


struct StepsBuilder<'a> {
    skip: bool,
    status: common::Status<cdata::Timeline<cdata::Step<'a>>>,
}
impl<'a> StepsBuilder<'a> {
    fn new() -> StepsBuilder<'a> {
        StepsBuilder {
            skip: false,
            status: common::Status { result: cdata::Timeline::new(), error: None } }
    }
}
impl<'a> ast::NodeVisitor<'a> for StepsBuilder<'a> {
    type Result = cdata::Timeline<cdata::Step<'a>>;
    fn end_visit_node(&mut self, node: &'a ast::Node<'a>) {
        if !self.skip {
            let step = cdata::Step { node: node, kind: cdata::StepKind::Unk };
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
