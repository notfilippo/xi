use std::str::Chars;

use miette::Result;
use peekmore::{PeekMore, PeekMoreIterator};
use rug::{Assign, Float, Integer};

use crate::{
    error::{MalformedNumber, UnexpectedCharacter, UnterminatedSequence},
    token::{Span, Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a str,
    chars: PeekMoreIterator<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

const FLOAT_PRECISION: u32 = 32;

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekmore(),
            tokens: vec![],
            start: 0,
            current: 0,
        }
    }

    fn span(&self) -> Span {
        Span::new(self.start, self.current - self.start)
    }

    fn emit(&mut self, kind: TokenKind) -> Result<()> {
        self.tokens.push(Token::new(kind, self.span()));
        Ok(())
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn peek_is(&mut self, f: fn(char) -> bool) -> bool {
        match self.peek() {
            Some(&c) => f(c),
            None => false,
        }
    }

    fn peek_nth(&mut self, n: usize) -> Option<char> {
        self.chars.advance_cursor_by(n);
        let result = self.chars.peek().map(|&a| a);
        self.chars.reset_cursor();
        return result;
    }

    fn peek_nth_is(&mut self, n: usize, f: fn(char) -> bool) -> bool {
        match self.peek_nth(n) {
            Some(c) => f(c),
            None => false,
        }
    }

    fn peek_and_match(&mut self, c: char) -> bool {
        match self.peek() {
            None => false,
            Some(&other) => {
                if c == other {
                    self.next();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        let next = self.chars.next();
        if next.is_some() {
            self.current += 1;
        }

        next
    }

    fn scan_string(&mut self) -> Result<()> {
        loop {
            match self.peek() {
                Some('"') | None => break,
                _ => self.next(),
            };
        }

        if !self.peek().is_none() {
            self.next(); // "
            let literal = self.source[self.start + 1..self.current - 1].to_string();
            self.emit(TokenKind::String(literal))?;
            Ok(())
        } else {
            Err(UnterminatedSequence {
                src: self.source.to_string(),
                span: self.span().into(),
            }
            .into())
        }
    }

    fn scan_number_as_float(&mut self) -> Result<()> {
        let literal = self.source[self.start..self.current].to_string();
        let parse = Float::parse(literal);

        if let Ok(src) = parse {
            let mut float = Float::new(FLOAT_PRECISION);
            float.assign(src);
            self.emit(TokenKind::Float(float))
        } else {
            Err(MalformedNumber {
                src: self.source.to_string(),
                span: self.span().into(),
            }
            .into())
        }
    }

    fn scan_number_as_integer(&mut self) -> Result<()> {
        let literal = self.source[self.start..self.current].to_string();
        let parse = Integer::parse(literal);

        if let Ok(src) = parse {
            let mut integer = Integer::new();
            integer.assign(src);
            self.emit(TokenKind::Integer(integer))
        } else {
            Err(MalformedNumber {
                src: self.source.to_string(),
                span: self.span().into(),
            }
            .into())
        }
    }

    fn scan_identifier(&mut self) -> Result<()> {
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            self.next();
        }

        let literal = &self.source[self.start..self.current];

        match literal {
            "and" => self.emit(TokenKind::And),
            "class" => self.emit(TokenKind::Class),
            "else" => self.emit(TokenKind::Else),
            "false" => self.emit(TokenKind::False),
            "fn" => self.emit(TokenKind::Fn),
            "for" => self.emit(TokenKind::For),
            "if" => self.emit(TokenKind::If),
            "nil" => self.emit(TokenKind::Nil),
            "or" => self.emit(TokenKind::Or),
            "print" => self.emit(TokenKind::Print),
            "return" => self.emit(TokenKind::Return),
            "super" => self.emit(TokenKind::Super),
            "this" => self.emit(TokenKind::This),
            "true" => self.emit(TokenKind::True),
            "var" => self.emit(TokenKind::Var),
            "while" => self.emit(TokenKind::While),
            other => self.emit(TokenKind::Identifier(other.to_string())),
        }
    }

    fn scan_number(&mut self) -> Result<()> {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.next();
        }

        if self.peek_is(|c| c == '.') {
            if self.peek_nth_is(1, |c| c.is_ascii_digit()) {
                self.next(); // .

                while self.peek_is(|c| c.is_ascii_digit()) {
                    self.next();
                }

                if self.peek_is(|c| c == 'e') {
                    if self.peek_nth_is(1, |c| c == '+' || c == '-') {
                        if self.peek_nth_is(2, |c| c.is_ascii_digit()) {
                            self.next(); // e
                        }
                    }

                    if self.peek_nth_is(1, |c| c.is_ascii_digit()) {
                        self.next(); // e (or + / - if e already removed)
                        while self.peek_is(|c| c.is_ascii_digit()) {
                            self.next();
                        }
                    }
                }

                return self.scan_number_as_float();
            }
        } else if self.peek_is(|c| c == 'e') {
            if self.peek_nth_is(1, |c| c == '+' || c == '-') {
                if self.peek_nth_is(2, |c| c.is_ascii_digit()) {
                    self.next(); // e
                }
            }

            if self.peek_nth_is(1, |c| c.is_ascii_digit()) {
                self.next(); // e (or + / - if e already removed)
                while self.peek_is(|c| c.is_ascii_digit()) {
                    self.next();
                }

                return self.scan_number_as_float();
            }
        }

        return self.scan_number_as_integer();
    }

    fn scan_token(&mut self, c: char) -> Result<()> {
        match c {
            '(' => self.emit(TokenKind::LeftParen),
            ')' => self.emit(TokenKind::RightParen),
            '{' => self.emit(TokenKind::LeftBrace),
            '}' => self.emit(TokenKind::RightBrace),
            ',' => self.emit(TokenKind::Comma),
            '.' => self.emit(TokenKind::Dot),
            '-' => self.emit(TokenKind::Minus),
            '+' => self.emit(TokenKind::Plus),
            ';' => self.emit(TokenKind::Semicolon),
            '*' => self.emit(TokenKind::Star),
            '/' => self.emit(TokenKind::Slash),
            '|' => self.emit(TokenKind::Pipe),
            '"' => self.scan_string(),
            '!' => {
                if self.peek_and_match('=') {
                    self.emit(TokenKind::BangEqual)
                } else {
                    self.emit(TokenKind::Bang)
                }
            }
            '=' => {
                if self.peek_and_match('=') {
                    self.emit(TokenKind::EqualEqual)
                } else {
                    self.emit(TokenKind::Equal)
                }
            }
            '>' => {
                if self.peek_and_match('=') {
                    self.emit(TokenKind::GreaterEqual)
                } else {
                    self.emit(TokenKind::Greater)
                }
            }
            '<' => {
                if self.peek_and_match('=') {
                    self.emit(TokenKind::LessEqual)
                } else {
                    self.emit(TokenKind::Less)
                }
            }
            '#' => {
                loop {
                    match self.next() {
                        None | Some('\n') => break,
                        _ => {}
                    }
                }
                Ok(())
            }
            ' ' | '\n' | '\r' | '\t' => Ok(()), // skip
            c => {
                if c.is_ascii_digit() {
                    self.scan_number()
                } else if c.is_ascii_alphabetic() {
                    self.scan_identifier()
                } else {
                    Err(UnexpectedCharacter {
                        src: self.source.to_string(),
                        span: self.span().into(),
                    }
                    .into())
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while let Some(c) = self.next() {
            self.scan_token(c)?;
            self.start = self.current;
        }

        self.emit(TokenKind::Eof)?;

        Ok(&self.tokens)
    }
}
