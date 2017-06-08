use common;
use ast;
use io;


pub fn parse_keyword_or_identifier(source: &mut io::SourceFlexIterator)
                                   -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert!(match source.peek(0).unwrap() {
                '_' | 'a'...'z' | 'A'...'Z' => true,
                _ => false,
            });

    let location = source.location();
    let mut value = String::new();
    while let Some(chr) = source.peek(0) {
        match chr {
            '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => {
                value.push(chr);
                source.next();
            }
            _ => break,
        }
    }
    let kind = match value.as_str() {
        "true" | "false" => ast::TokenKind::Boolean,
        "print" => ast::TokenKind::Print,
        _ => ast::TokenKind::Identifier,
    };

    let token = ast::Token {
        kind: kind,
        value: value,
        location: location,
    };
    common::Status {
        result: token,
        error: None,
    }
}
