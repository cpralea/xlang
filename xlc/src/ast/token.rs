use common;


pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: common::Location,
}


impl Token {
    pub fn new(kind: TokenKind, value: String, location: common::Location) -> Token {
        Token {
            kind: kind,
            value: value,
            location: location,
        }
    }
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
    Not,
    Or,
    String,
    Print,
    RParen,
    Separator,
    Sub,
    Unknown,
}
