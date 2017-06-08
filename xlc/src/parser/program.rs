use common;
use ast;

use super::assignment;
use super::print;
use super::unknown;


pub fn parse_program<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                         -> common::Status<ast::Node<'a>> {
    let (mut statements,) = (ast::Nodes::new(),);

    while let Some(token) = tokens.peek(0) {
        match token.kind {
            ast::TokenKind::Separator => {
                tokens.next();
                continue;
            }
            ast::TokenKind::Identifier => {
                let status = assignment::parse_assignment(tokens);
                statements.push(status.result);
                if status.error.is_some() {
                    return make_program(None, statements).error(status.error);
                }
            }
            ast::TokenKind::Print => {
                let status = print::parse_print(tokens);
                statements.push(status.result);
                if status.error.is_some() {
                    return make_program(None, statements).error(status.error);
                }
            }
            _ => {
                let status = unknown::parse_unknown(tokens);
                return make_program(None, statements).error(status.error);
            }
        }
    }

    make_program(None, statements)
}


fn make_program<'a>(token: Option<&'a ast::Token>,
                    statements: ast::Nodes<'a>)
                    -> common::Status<ast::Node<'a>> {
    let program = Box::new(ast::NodeKind::Program { statements: statements });
    common::Status {
        result: ast::Node::new(program, token),
        error: None,
    }
}
