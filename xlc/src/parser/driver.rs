use common;
use ast;

use super::program;


pub fn parse<'a>(tokens: &'a ast::Tokens) -> common::Status<ast::Node<'a>> {
    program::parse_program(&mut tokens.iter_flex())
}
