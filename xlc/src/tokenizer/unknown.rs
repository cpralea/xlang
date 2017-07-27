use common;
use ast;
use io;


pub fn parse_unknown(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());

    let location = source.location();
    let value = vec![source.peek(0).unwrap()].into_iter().collect();
    source.next();

    let token = ast::Token::new(ast::TokenKind::Unknown, value, location);
    let error = Some(common::Error {
        location: Some(token.location),
        message: format!("Unknown token '{}'.", token.value),
    });
    common::Status {
        result: token,
        error: error,
    }
}
