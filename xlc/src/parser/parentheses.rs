use common;
use ast;

use super::utils;


pub fn parse_lparen<'a>(
    tokens: &mut common::FlexIteratorByRef<'a, ast::Token>,
) -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens, Some("'('"), hashset!{ ast::TokenKind::LParen })
}


pub fn parse_rparen<'a>(
    tokens: &mut common::FlexIteratorByRef<'a, ast::Token>,
) -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens, Some("')'"), hashset!{ ast::TokenKind::RParen })
}
