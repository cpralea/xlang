use common;
use ast;
use io;

use common::FlexIteratorByVal;


pub fn tokenize(source: &io::Source) -> common::Status<ast::Tokens> {
    let source = &mut source.iter_flex();

    let mut tokens = ast::Tokens::new();
    let mut error = None;

    loop {
        let chr = source.peek(0);
        if chr.is_none() || error.is_some() {
            break;
        }

        let status;
        match chr.unwrap() {
            ' '|'\r'|'\t'           => { source.next(); continue; }
            '='                     => status = parse_assign(source),
            '_'|'a'...'z'|'A'...'Z' => status = parse_keyword_or_identifier(source),
            '0'...'9'               => status = parse_integer(source),
            '"'                     => status = parse_string(source),
            '\n'                    => status = parse_separator(source),
            _                       => status = parse_unknown(source),
        }
        tokens.push(status.result);
        error = status.error;
    }

    common::Status { result: tokens, error: error }
}


fn parse_assign(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert_eq!(source.peek(0).unwrap(), '=');


    let location = source.location();
    let value = String::from("=");
    source.next();

    let token = ast::Token { kind: ast::TokenKind::Assign, value: value, location: location };
    common::Status { result: token, error: None }
}


fn parse_keyword_or_identifier(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert!(match source.peek(0).unwrap() {
        '_'|'a'...'z'|'A'...'Z' => true,
        _ => false
    });

    let location = source.location();
    let mut value = String::new();
    while let Some(chr) = source.peek(0) {
        match chr {
            '_'|'a'...'z'|'A'...'Z'|'0'...'9' => {
                value.push(chr);
                source.next();
            }
            _ => break,
        }}
    let kind = match value.as_str() {
        "print" => ast::TokenKind::Print,
        _       => ast::TokenKind::Identifier,
    };

    let token = ast::Token { kind: kind, value: value, location: location };
    common::Status { result: token, error: None }
}


fn parse_integer(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert!(match source.peek(0).unwrap() {
        '0'...'9' => true,
        _ => false
    });

    let location = source.location();
    let mut value = String::new();
    while let Some(chr) = source.peek(0) {
        if '0' <= chr && chr <= '9' {
            value.push(chr);
            source.next();
        } else { break; } }

    let token = ast::Token { kind: ast::TokenKind::Integer, value: value, location: location };
    let error = match token.value.len() > 1 && token.value.starts_with("0") {
        true => Some(common::Error { location: Some(token.location),
            message: format!("Invalid integer '{}'.", token.value) }),
        false => None,
    };
    common::Status { result: token, error: error }
}


fn parse_string(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
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
                        'n'|'r'|'t'|'"'|'\\' => {
                            value.push(chr);
                            source.next();
                        }
                        _ => {
                            error = Some(common::Error { location: Some(source.location()),
                                message: format!("Invalid escape sequence '\\{}'.", next_chr) });
                            break;
                        }
                    }
                    chr = next_chr;
                }
            }
            value.push(chr);
            source.next();
        } else { break; } }

    let token = ast::Token { kind: ast::TokenKind::String, value: value, location: location };
    error = error.or(match source.peek(0) {
        None => Some(common::Error { location: Some(token.location),
            message: format!("Invalid string '{}'.", token.value) }),
        Some(_) => None,
    });
    source.next();
    common::Status { result: token, error: error }
}


fn parse_separator(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());
    assert_eq!(source.peek(0).unwrap(), '\n');

    let location = source.location();
    let value = String::from("\\n");
    source.next();

    let token = ast::Token { kind: ast::TokenKind::Separator, value: value, location: location };
    common::Status { result: token, error: None }
}


fn parse_unknown(source: &mut io::SourceFlexIterator) -> common::Status<ast::Token> {
    assert!(source.peek(0).is_some());

    let location = source.location();
    let value = vec![source.peek(0).unwrap()].into_iter().collect();
    source.next();

    let token = ast::Token { kind: ast::TokenKind::Unknown, value: value, location: location };
    let error = Some(common::Error { location: Some(token.location),
        message: format!("Unknown token '{}'.", token.value) });
    common::Status { result: token, error: error }
}
