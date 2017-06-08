use common;
use ast;

use super::utils;


pub fn parse_identifier<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                            -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens, Some("Identifier"), hashset!{ ast::TokenKind::Identifier })
}
