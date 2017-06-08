use ast;


pub fn dump_token(token: &ast::Token) -> String {
    format!("(Token: Value: '{}', {:?}@{})", token.value, token.kind, token.location)
}
