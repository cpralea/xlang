use common;
use ast;
use io;


pub fn parse_symbol(source: &mut io::SourceFlexIterator,
                    symbol: &str,
                    kind: ast::TokenKind)
                    -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert_eq!(source.peek(0).unwrap(), symbol.chars().next().unwrap());

    let location = source.location();
    let mut value = String::new();
    for chr in symbol.chars() {
        if source.peek(0) == Some(chr) {
            value.push(chr);
            source.next();
        } else {
            break;
        }
    }

    let token = ast::Token {
        kind: kind,
        value: value,
        location: location,
    };
    let error = match token.value != symbol {
        true => {
            Some(common::Error {
                     location: Some(token.location),
                     message: format!("Expected '{}'. Found '{}'.", symbol, token.value),
                 })
        }
        false => None,
    };
    common::Status {
        result: token,
        error: error,
    }
}
