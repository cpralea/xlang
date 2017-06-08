use std::collections;

use common;
use ast;


pub fn next_token<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>,
                      error: Option<&str>,
                      kinds: collections::HashSet<ast::TokenKind>)
                      -> common::Status<Option<&'a ast::Token>> {
    let mut status = peek_token(tokens, 0, kinds);

    if status.error.is_none() {
        tokens.next();
    } else {
        if let Some(error) = error {
            status.error = match status.result {
                Some(token) => {
                    status
                        .error
                        .map(|mut e| {
                                 e.message = format!("Expected {}. Found {}.",
                                                     error,
                                                     pretty_token(&token.value));
                                 e
                             })
                }
                None => {
                    status
                        .error
                        .map(|mut e| {
                                 e.message = format!("Expected {}. Found End of File.", error);
                                 e
                             })
                }
            }
        }
    }

    status
}


fn peek_token<'a>(tokens: &common::FlexIteratorByRef<'a, ast::Token>,
                  offset: usize,
                  kinds: collections::HashSet<ast::TokenKind>)
                  -> common::Status<Option<&'a ast::Token>> {
    let token = tokens.peek(offset);
    let location = token.map(|token| token.location);
    let error = match token {
        Some(token) => {
            match kinds.is_empty() {
                false if !kinds.contains(&token.kind) => {
                    Some(common::Error {
                             location: location,
                             message: format!("Unexpected token {}.", pretty_token(&token.value)),
                         })
                }
                _ => None,
            }
        }
        None => {
            Some(common::Error {
                     location: location,
                     message: format!("Unexpected End of File."),
                 })
        }
    };

    common::Status {
        result: token,
        error: error,
    }
}


fn pretty_token(value: &String) -> String {
    match value.as_str() {
        "\\n" => String::from("End of Statement"),
        _ => format!("'{}'", value),
    }
}
