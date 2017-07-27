use common;
use ast;
use io;

use super::literals;
use super::operators;
use super::parentheses;
use super::separator;
use super::unknown;
use super::words;

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
            ' ' | '\r' | '\t' => {
                source.next();
                continue;
            }
            '=' if source.peek(1) != Some('=') => status = operators::parse_assign(source),
            '+' => status = operators::parse_add(source),
            '-' => status = operators::parse_sub(source),
            '*' => status = operators::parse_mul(source),
            '/' => status = operators::parse_div(source),
            '!' if source.peek(1) != Some('=') => status = operators::parse_not(source),
            '|' => status = operators::parse_or(source),
            '&' => status = operators::parse_and(source),
            '=' if source.peek(1) == Some('=') => status = operators::parse_eq(source),
            '!' if source.peek(1) == Some('=') => status = operators::parse_ne(source),
            '<' if source.peek(1) != Some('=') => status = operators::parse_lt(source),
            '<' if source.peek(1) == Some('=') => status = operators::parse_le(source),
            '>' if source.peek(1) != Some('=') => status = operators::parse_gt(source),
            '>' if source.peek(1) == Some('=') => status = operators::parse_ge(source),
            '(' => status = parentheses::parse_lparen(source),
            ')' => status = parentheses::parse_rparen(source),
            '_' | 'a'...'z' | 'A'...'Z' => status = words::parse_keyword_or_identifier(source),
            '0'...'9' => status = literals::parse_integer(source),
            '"' => status = literals::parse_string(source),
            '\n' => status = separator::parse_separator(source),
            _ => status = unknown::parse_unknown(source),
        }
        tokens.push(status.result);
        error = status.error;
    }

    common::Status {
        result: tokens,
        error: error,
    }
}
