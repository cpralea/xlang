use common;


pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: common::Location,
}


pub type Tokens = common::Collection<Token>;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Add,
    And,
    Assign,
    Boolean,
    Div,
    Eq,
    Ge,
    Gt,
    Identifier,
    Integer,
    Le,
    LParen,
    Lt,
    Mul,
    Ne,
    Or,
    String,
    Print,
    RParen,
    Separator,
    Sub,
    Unknown,
}
