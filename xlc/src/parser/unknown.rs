use common;
use ast;

use super::utils;


pub fn parse_unknown<'a>(tokens: &mut common::FlexIteratorByRef<'a, ast::Token>)
                         -> common::Status<ast::Node<'a>> {
    let head;

    let status = utils::next_token(tokens, None, hashset!{});
    if status.error.is_some() {
        return make_unknown(status.result).error(status.error);
    }
    head = status.result;

    make_unknown(head).error(Some(common::Error {
                                      location: Some(head.unwrap().location),
                                      message: format!("Unexpected token '{}'.",
                                                       head.unwrap().value),
                                  }))
}


fn make_unknown<'a>(token: Option<&'a ast::Token>) -> common::Status<ast::Node<'a>> {
    let kind = Box::new(ast::NodeKind::Unknown);
    common::Status {
        result: ast::Node::new(kind, token),
        error: None,
    }
}
