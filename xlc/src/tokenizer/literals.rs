use common;
use ast;
use io;


pub fn parse_integer(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert!(match source.peek(0).unwrap() {
        '0'...'9' => true,
        _ => false,
    });

    let location = source.location();
    let mut value = String::new();
    while let Some(chr) = source.peek(0) {
        if '0' <= chr && chr <= '9' {
            value.push(chr);
            source.next();
        } else {
            break;
        }
    }

    let token = ast::Token::new(ast::TokenKind::Integer, value, location);
    let error = match token.value.len() > 1 && token.value.starts_with("0") {
        true => {
            Some(common::Error {
                location: Some(token.location),
                message: format!("Invalid integer '{}'.", token.value),
            })
        }
        false => None,
    };
    common::Status {
        result: token,
        error: error,
    }
}


pub fn parse_string(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert_eq!(source.peek(0).unwrap(), '"');

    let mut error = None;

    let location = source.location();
    let mut value = String::new();
    source.next();
    while let Some(mut chr) = source.peek(0) {
        if chr != '"' {
            if chr == '\\' {
                if let Some(next_chr) = source.peek(1) {
                    match next_chr {
                        'n' | 'r' | 't' | '"' | '\\' => {
                            value.push(chr);
                            source.next();
                        }
                        _ => {
                            error = Some(common::Error {
                                location: Some(source.location()),
                                message: format!("Invalid escape sequence '\\{}'.", next_chr),
                            });
                            break;
                        }
                    }
                    chr = next_chr;
                }
            }
            value.push(chr);
            source.next();
        } else {
            break;
        }
    }

    let token = ast::Token::new(ast::TokenKind::String, value, location);
    error = error.or(match source.peek(0) {
        None => {
            Some(common::Error {
                location: Some(token.location),
                message: format!("Invalid string '{}'.", token.value),
            })
        }
        Some(_) => None,
    });
    source.next();
    common::Status {
        result: token,
        error: error,
    }
}
