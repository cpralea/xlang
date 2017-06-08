use common;
use ast;

use super::expression;
use super::statement;
use super::utils;


pub fn parse_print<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                       -> common::Status<ast::Node<'a>> {
    let head;
    let (mut expression,) = (None,);

    let status = parse_print_keyword(tokens);
    if status.error.is_some() {
        return make_print(status.result, expression).error(status.error);
    }
    head = status.result;

    let status = expression::parse_expression(tokens);
    if status.error.is_some() {
        return make_print(head, expression).error(status.error);
    }
    expression = Some(status.result);

    let status = statement::parse_end_of_statement(tokens);
    if status.error.is_some() {
        return make_print(head, expression).error(status.error);
    }

    make_print(head, expression)
}


fn parse_print_keyword<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                           -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens, Some("'print'"), hashset!{ ast::TokenKind::Print })
}


fn make_print<'a>(token: Option<&'a ast::Token>,
                  expression: Option<ast::Node<'a>>)
                  -> common::Status<ast::Node<'a>> {
    let kind = Box::new(ast::NodeKind::Print { expression: expression });
    common::Status {
        result: ast::Node::new(kind, token),
        error: None,
    }
}
