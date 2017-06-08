use common;
use ast;

use super::expression;
use super::identifier;
use super::statement;
use super::utils;


pub fn parse_assignment<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                            -> common::Status<ast::Node<'a>> {
    let head;
    let (mut identifier, mut expression) = (None, None);

    let status = identifier::parse_identifier(tokens);
    if status.error.is_some() {
        return make_assignment(status.result, identifier, expression).error(status.error);
    }
    head = status.result;
    identifier = Some(status.result.unwrap().value.clone());

    let status = parse_assign_operator(tokens);
    if status.error.is_some() {
        return make_assignment(head, identifier, expression).error(status.error);
    }

    let status = expression::parse_expression(tokens);
    if status.error.is_some() {
        return make_assignment(head, identifier, expression).error(status.error);
    }
    expression = Some(status.result);

    let status = statement::parse_end_of_statement(tokens);
    if status.error.is_some() {
        return make_assignment(head, identifier, expression).error(status.error);
    }

    make_assignment(head, identifier, expression)
}


fn parse_assign_operator<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                             -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens, Some("'='"), hashset!{ ast::TokenKind::Assign })
}


fn make_assignment<'a>(token: Option<&'a ast::Token>,
                       identifier: Option<String>,
                       expression: Option<ast::Node<'a>>)
                       -> common::Status<ast::Node<'a>> {
    let kind = Box::new(ast::NodeKind::Assignment {
                            identifier: identifier,
                            expression: expression,
                        });
    common::Status {
        result: ast::Node::new(kind, token),
        error: None,
    }
}
