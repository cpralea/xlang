use common;
use ast;

use super::utils;


pub fn parse_end_of_statement<'a>(
    tokens: &mut common::FlexIteratorByRef<'a, ast::Token>,
) -> common::Status<Option<&'a ast::Token>> {
    utils::next_token(tokens, Some("End of Statement"), hashset!{ ast::TokenKind::Separator })
}
