use common;
use ast;
use io;

use super::utils;


pub fn parse_assign(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "=", ast::TokenKind::Assign)
}


pub fn parse_add(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "+", ast::TokenKind::Add)
}


pub fn parse_sub(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "-", ast::TokenKind::Sub)
}


pub fn parse_mul(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "*", ast::TokenKind::Mul)
}


pub fn parse_div(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "/", ast::TokenKind::Div)
}


pub fn parse_or(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "||", ast::TokenKind::Or)
}


pub fn parse_and(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "&&", ast::TokenKind::And)
}
