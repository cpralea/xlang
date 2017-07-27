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


pub fn parse_not(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "!", ast::TokenKind::Not)
}


pub fn parse_or(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "||", ast::TokenKind::Or)
}


pub fn parse_and(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "&&", ast::TokenKind::And)
}


pub fn parse_eq(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "==", ast::TokenKind::Eq)
}


pub fn parse_ne(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "!=", ast::TokenKind::Ne)
}


pub fn parse_lt(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "<", ast::TokenKind::Lt)
}


pub fn parse_le(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, "<=", ast::TokenKind::Le)
}


pub fn parse_gt(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, ">", ast::TokenKind::Gt)
}


pub fn parse_ge(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    utils::parse_symbol(source, ">=", ast::TokenKind::Ge)
}
