use std::collections;

use common;
use ast;


pub fn parse<'a>(
           tokens: &'a ast::Tokens)
        -> common::Status<ast::Node<'a>> {

    parse_program(&mut tokens.iter_flex())
}


fn parse_program<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<ast::Node<'a>> {

    let (mut statements, ) = (ast::Nodes::new(), );

    while let Some(token) = tokens.peek(0) { match token.kind {
        ast::TokenKind::Separator => {
            tokens.next();
            continue;
        }
        ast::TokenKind::Identifier => {
            let status = parse_assignment(tokens);
            statements.push(status.result);
            if status.error.is_some() {
                return make_program(None, statements).error(status.error);
            }
        }
        ast::TokenKind::Print => {
            let status = parse_print(tokens);
            statements.push(status.result);
            if status.error.is_some() {
                return make_program(None, statements).error(status.error);
            }
        }
        _ => {
            let status = parse_unknown(tokens);
            return make_program(None, statements).error(status.error);
        }
    } }

    make_program(None, statements)
}


fn parse_assignment<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<ast::Node<'a>> {

    let head;
    let (mut identifier, mut expression) = (None, None);

    let status = parse_identifier(tokens);
    if status.error.is_some() {
        return make_assignment(status.result, identifier, expression).error(status.error);
    }
    head = status.result;
    identifier = Some(status.result.unwrap().value.clone());

    let status = parse_assign_operator(tokens);
    if status.error.is_some() {
        return make_assignment(head, identifier, expression).error(status.error);
    }

    let status = parse_expression(tokens);
    if status.error.is_some() {
        return make_assignment(head, identifier, expression).error(status.error);
    }
    expression = Some(status.result);

    let status = parse_end_of_statement(tokens);
    if status.error.is_some() {
        return make_assignment(head, identifier, expression).error(status.error);
    }

    make_assignment(head, identifier, expression)
}


fn parse_print<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<ast::Node<'a>> {

    let head;
    let (mut expression, ) = (None, );

    let status = parse_print_keyword(tokens);
    if status.error.is_some() {
        return make_print(status.result, expression).error(status.error);
    }
    head = status.result;

    let status = parse_expression(tokens);
    if status.error.is_some() {
        return make_print(head, expression).error(status.error);
    }
    expression = Some(status.result);

    let status = parse_end_of_statement(tokens);
    if status.error.is_some() {
        return make_print(head, expression).error(status.error);
    }

    make_print(head, expression)
}


fn parse_expression<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<ast::Node<'a>> {

    let head;
    let (mut boolean, mut identifier, mut integer, mut string) = (None, None, None, None);

    let status = parse_expression_start(tokens);
    if status.error.is_some() {
        return make_expression(status.result, boolean, identifier, integer, string)
                    .error(status.error);
    }
    head = status.result;
    match status.result.unwrap().kind {
        ast::TokenKind::Boolean =>
            boolean = Some(status.result.unwrap().value.parse::<bool>().unwrap()),
        ast::TokenKind::Identifier =>
            identifier = Some(status.result.unwrap().value.clone()),
        ast::TokenKind::Integer =>
            integer = Some(status.result.unwrap().value.parse::<i64>().unwrap()),
        ast::TokenKind::String =>
            string = Some(status.result.unwrap().value.clone()),
        _ => unreachable!(),
    }

    make_expression(head, boolean, identifier, integer, string)
}


fn parse_unknown<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<ast::Node<'a>> {

    let head;

    let status = next_token(tokens, None, hashset!{});
    if status.error.is_some() {
        return make_unknown(status.result).error(status.error);
    }
    head = status.result;

    make_unknown(head).error(Some(common::Error { location: Some(head.unwrap().location),
        message: format!("Unexpected token '{}'.", head.unwrap().value) }))
}


fn make_program<'a>(
           token: Option<&'a ast::Token>,
           statements: ast::Nodes<'a>)
        -> common::Status<ast::Node<'a>> {

    let program = Box::new(ast::NodeKind::Program {
        statements: statements });
    common::Status { result: ast::Node::new(program, token), error: None }
}


fn make_assignment<'a>(
           token: Option<&'a ast::Token>,
           identifier: Option<String>,
           expression: Option<ast::Node<'a>>)
        -> common::Status<ast::Node<'a>> {

    let kind = Box::new(ast::NodeKind::Assignment {
        identifier: identifier, expression: expression });
    common::Status { result: ast::Node::new(kind, token), error: None }
}


fn make_print<'a>(
           token: Option<&'a ast::Token>,
           expression: Option<ast::Node<'a>>)
        -> common::Status<ast::Node<'a>> {

    let kind = Box::new(ast::NodeKind::Print {
        expression: expression });
    common::Status { result: ast::Node::new(kind, token), error: None }
}


fn make_expression<'a>(
           token: Option<&'a ast::Token>,
           boolean: Option<bool>,
           identifier: Option<String>,
           integer: Option<i64>,
           string: Option<String>)
        -> common::Status<ast::Node<'a>> {

    let kind = Box::new(ast::NodeKind::Expression {
        boolean: boolean, identifier: identifier, integer: integer, string: string });
    common::Status { result: ast::Node::new(kind, token), error: None }
}


fn make_unknown<'a>(
           token: Option<&'a ast::Token>)
        -> common::Status<ast::Node<'a>> {

    let kind = Box::new(ast::NodeKind::Unknown);
    common::Status { result: ast::Node::new(kind, token), error: None }
}


fn parse_identifier<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<Option<&'a ast::Token>> {

    next_token(tokens, Some("Identifier"), hashset!{
        ast::TokenKind::Identifier })
}


fn parse_print_keyword<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<Option<&'a ast::Token>> {

    next_token(tokens, Some("'print'"), hashset!{
        ast::TokenKind::Print })
}


fn parse_assign_operator<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<Option<&'a ast::Token>> {

    next_token(tokens, Some("'='"), hashset!{
        ast::TokenKind::Assign })
}


fn parse_expression_start<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<Option<&'a ast::Token>> {

    next_token(tokens, Some("Identifier, Boolean, Integer or String"), hashset!{
        ast::TokenKind::Identifier,
        ast::TokenKind::Boolean, ast::TokenKind::Integer, ast::TokenKind::String })
}


fn parse_end_of_statement<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
        -> common::Status<Option<&'a ast::Token>> {

    next_token(tokens, Some("End of Statement"), hashset!{
        ast::TokenKind::Separator })
}


fn next_token<'a>(
           tokens: &mut common::FlexIteratorByRef<'a, ast::Token>,
           error: Option<&str>,
           kinds: collections::HashSet<ast::TokenKind>)
        -> common::Status<Option<&'a ast::Token>> {

    let mut status = peek_token(tokens, 0, kinds);

    if status.error.is_none() {
        tokens.next();
    } else {
        if let Some(error) = error {
            status.error = match status.result {
                Some(token) => status.error.map(|mut e| {
                    e.message = format!("Expected {}. Found {}.", error, pretty_token(&token.value)); e }),
                None => status.error.map(|mut e| {
                    e.message = format!("Expected {}. Found End of File.", error); e }), }}}

    status
}


fn peek_token<'a>(
           tokens: &common::FlexIteratorByRef<'a, ast::Token>,
           offset: usize,
           kinds: collections::HashSet<ast::TokenKind>)
        -> common::Status<Option<&'a ast::Token>> {

    let token = tokens.peek(offset);
    let location = token.map(|token| token.location);
    let error = match token {
        Some(token) => match kinds.is_empty() {
            false if !kinds.contains(&token.kind) =>
                Some(common::Error { location: location,
                    message: format!("Unexpected token {}.", pretty_token(&token.value)) }),
            _ => None,
        },
        None => Some(common::Error { location: location,
            message: format!("Unexpected End of File.") }),
    };

    common::Status { result: token, error: error }
}


fn pretty_token(
           value: &String)
        -> String {

    match value.as_str() {
        "\\n" => String::from("End of Statement"),
        _ => format!("'{}'", value),
    }
}
