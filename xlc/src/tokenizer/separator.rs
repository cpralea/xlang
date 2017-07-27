use common;
use ast;
use io;


pub fn parse_separator(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert_eq!(source.peek(0).unwrap(), '\n');

    let location = source.location();
    let value = String::from("\\n");
    source.next();

    let token = ast::Token::new(ast::TokenKind::Separator, value, location);
    common::Status {
        result: token,
        error: None,
    }
}
