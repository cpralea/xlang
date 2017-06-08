use common;
use ast;

use super::parentheses;
use super::utils;


pub fn parse_expression<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                            -> common::Status<ast::Node<'a>> {
    parse_expression_l2(tokens)
}


fn parse_expression_l2<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                           -> common::Status<ast::Node<'a>> {
    let head;
    let (operator, left, right);

    let status = parse_expression_l1(tokens);
    if status.error.is_some() {
        return status;
    }
    left = Some(status.result);

    let status = parse_expression_l2_operator(tokens);
    if status.error.is_some() {
        return common::Status {
                   result: left.unwrap(),
                   error: None,
               };
    }
    head = status.result;
    operator = Some(status.result.unwrap().value.clone());

    let status = parse_expression_l2(tokens);
    if status.error.is_some() {
        return status;
    }
    right = Some(status.result);

    make_expression(head, None, None, None, None, operator, left, right)
}


fn parse_expression_l1<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                           -> common::Status<ast::Node<'a>> {
    let head;
    let (operator, left, right);

    let status = parse_expression_l0(tokens);
    if status.error.is_some() {
        return status;
    }
    left = Some(status.result);

    let status = parse_expression_l1_operator(tokens);
    if status.error.is_some() {
        return common::Status {
                   result: left.unwrap(),
                   error: None,
               };
    }
    head = status.result;
    operator = Some(status.result.unwrap().value.clone());

    let status = parse_expression_l1(tokens);
    if status.error.is_some() {
        return status;
    }
    right = Some(status.result);

    make_expression(head, None, None, None, None, operator, left, right)
}


fn parse_expression_l0<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                           -> common::Status<ast::Node<'a>> {
    match tokens.peek(0) {
        Some(token) if token.kind == ast::TokenKind::LParen => {
            parse_expression_l2_inside_parens(tokens)
        }
        _ => parse_expression_l0_proper(tokens),
    }
}


fn parse_expression_l2_inside_parens<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                                         -> common::Status<ast::Node<'a>> {
    let status = parentheses::parse_lparen(tokens);
    assert!(status.error.is_none());

    let noparen_status = parse_expression_l2(tokens);
    if noparen_status.error.is_some() {
        return noparen_status;
    }

    let status = parentheses::parse_rparen(tokens);
    if status.error.is_some() {
        return common::Status {
                   result: noparen_status.result,
                   error: status.error,
               };
    }

    noparen_status
}


fn parse_expression_l0_proper<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                                  -> common::Status<ast::Node<'a>> {
    let head;
    let (mut boolean, mut identifier, mut integer, mut string) = (None, None, None, None);

    let status = parse_expression_l0_operand(tokens);
    if status.error.is_some() {
        return make_expression(status.result,
                               boolean,
                               identifier,
                               integer,
                               string,
                               None,
                               None,
                               None)
                       .error(status.error);
    }
    head = status.result;
    match status.result.unwrap().kind {
        ast::TokenKind::Boolean => {
            boolean = Some(status.result.unwrap().value.parse::<bool>().unwrap());
        }
        ast::TokenKind::Identifier => {
            identifier = Some(status.result.unwrap().value.clone());
        }
        ast::TokenKind::Integer => {
            integer = Some(status.result.unwrap().value.parse::<i64>().unwrap());
        }
        ast::TokenKind::String => {
            string = Some(status.result.unwrap().value.clone());
        }
        _ => unreachable!(),
    }

    make_expression(head, boolean, identifier, integer, string, None, None, None)
}


fn parse_expression_l0_operand<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                                   -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens,
                      Some("operand"),
                      hashset!{
                          ast::TokenKind::Identifier,
                          ast::TokenKind::Boolean,
                          ast::TokenKind::Integer,
                          ast::TokenKind::String })
}


fn parse_expression_l1_operator<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                                    -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens,
                      Some("'*', '/' or '&&'"),
                      hashset!{ ast::TokenKind::Mul, ast::TokenKind::Div, ast::TokenKind::And })
}


fn parse_expression_l2_operator<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                                    -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens,
                      Some("'+', '-' or '||'"),
                      hashset!{ ast::TokenKind::Add, ast::TokenKind::Sub, ast::TokenKind::Or })
}


fn make_expression<'a>(token: Option<&'a ast::Token>,
                       boolean: Option<bool>,
                       identifier: Option<String>,
                       integer: Option<i64>,
                       string: Option<String>,
                       operator: Option<String>,
                       left: Option<ast::Node<'a>>,
                       right: Option<ast::Node<'a>>)
                       -> common::Status<ast::Node<'a>> {
    let kind = Box::new(ast::NodeKind::Expression {
                            boolean: boolean,
                            identifier: identifier,
                            integer: integer,
                            string: string,
                            operator: operator,
                            left: left,
                            right: right,
                        });
    common::Status {
        result: ast::Node::new(kind, token),
        error: None,
    }
}
