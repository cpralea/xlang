use common;
use ast;
use io;

use super::utils;


pub fn parse_lparen(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "(", ast::TokenKind::LParen)
}


pub fn parse_rparen(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, ")", ast::TokenKind::RParen)
}
