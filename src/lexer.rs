use crate::token::{Token, TokenKind};

#[derive(Eq, PartialEq, Debug)]
pub enum LexerErrKind {
    UnknownToken,
    UnexpectedToken(char),
}

#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrKind,
    pub c: char,
    pub line: i32,
    pub col: i32,
}

impl LexerError {
    pub fn new(kind: LexerErrKind, c: char, line: i32, col: i32) -> LexerError {
        LexerError { kind, c, line, col }
    }

    pub fn generate_msg(&self) -> String {
        return match self.kind {
            LexerErrKind::UnknownToken => String::from(format!(
                "Unknown token [{}] @ {}:{}",
                self.c, self.line, self.col
            )),
            LexerErrKind::UnexpectedToken(c) => String::from(format!(
                "Unexpected token [{}] expected [{}] @ {}:{}",
                self.c, c, self.line, self.col
            )),
        };
    }
}

#[derive(Default, Debug)]
pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    line: i32,
    col: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            line: 1,
            ..Default::default()
        }
    }

    fn make_token(&self, kind: TokenKind, lexeme: Option<String>) -> Token {
        return Token::new(kind, lexeme, self.line, self.col);
    }

    fn peek(&self) -> char {
        return self.peekn(0);
    }

    fn peekn(&self, n: usize) -> char {
        if self.bound() {
            return '\0';
        }

        return self.src.as_bytes()[self.pos + n] as char;
    }

    fn consume(&mut self) -> char {
        let c: char = self.peek();

        self.advance();

        return c;
    }

    fn advance(&mut self) -> () {
        let c = self.peek();
        self.pos += 1;
        self.col += 1;

        if c == '\n' {
            self.line += 1;
            self.col = 1;
        }
    }

    fn bound(&self) -> bool {
        return self.pos > self.src.chars().count() - 1;
    }

    // TODO Handle errors in these functions

    fn make_symbol(&mut self) -> Result<Token, LexerError> {
        let mut str: String = String::new();
        str.push(self.src.as_bytes()[self.pos - 1] as char);

        while !self.bound() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            str.push(self.consume());
        }

        let strlen = str.len() as i32;
        let mut tok = self.make_token(TokenKind::Symbol, Some(str));
        tok.col -= strlen - 1;
        return Ok(tok);
    }

    fn make_number(&mut self) -> Result<Token, LexerError> {
        let mut str: String = String::new();
        str.push(self.src.as_bytes()[self.pos - 1] as char);

        while !self.bound() && self.peek().is_digit(10) {
            str.push(self.consume());
        }

        let strlen = str.len() as i32;
        let mut tok = self.make_token(TokenKind::Number, Some(str));
        tok.col -= strlen - 1;
        return Ok(tok);
    }

    fn make_string(&mut self) -> Result<Token, LexerError> {
        let mut str: String = String::new();
        let quote_index = self.col;

        while !self.bound() && self.peek() != '\"' {
            str.push(self.consume());
        }

        if self.peek() != '\"' {
            return Err(LexerError::new(
                LexerErrKind::UnexpectedToken('\"'),
                self.peek(),
                self.line,
                quote_index,
            ));
        }

        self.consume();

        let strlen = str.len() as i32;
        let mut tok = self.make_token(TokenKind::String, Some(str));
        tok.col -= strlen - 1;
        return Ok(tok);
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        if self.bound() {
            return Ok(self.make_token(TokenKind::Eof, None));
        }

        let c: char = self.consume();

        if c.is_whitespace() {
            return self.next_token();
        } else if c.is_digit(10) {
            return self.make_number();
        } else if c == '\"' {
            return self.make_string();
        } else if c == ';' {
            while !self.bound() && self.peek() != '\n' {
                self.consume();
            }

            return self.next_token();
        } else if c == '(' {
            return Ok(self.make_token(TokenKind::OpenParen, None));
        } else if c == ')' {
            return Ok(self.make_token(TokenKind::CloseParen, None));
        } else if c.is_ascii() {
            return self.make_symbol();
        }

        return Err(LexerError::new(
            LexerErrKind::UnknownToken,
            self.peek(),
            self.line,
            self.col,
        ));
    }
}
