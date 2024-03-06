#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    String,
    Number,
    Symbol,
    OpenParen,
    CloseParen,
    Tick,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,

    pub line: i32,
    pub col: i32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: Option<String>, line: i32, col: i32) -> Self {
        Token {
            kind,
            lexeme,
            line,
            col,
        }
    }
}
