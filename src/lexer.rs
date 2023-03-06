use std::str::Chars;

use miette::Result;
use peekmore::{PeekMore, PeekMoreIterator};
use rug::{Assign, Float, Integer};

use crate::{
    report::{MalformedFloatPrecision, MalformedNumber, UnexpectedCharacter, UnterminatedSequence},
    token::{Literal, Span, Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a str,
    chars: PeekMoreIterator<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

const DEFAULT_FLOAT_PRECISION: u32 = 64;

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

    fn emit(&mut self, kind: TokenKind, literal: Option<Literal>) -> Result<()> {
        self.tokens.push(Token::new(kind, literal, self.span()));
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

    fn matches(&mut self, c: char) -> bool {
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
            self.emit(TokenKind::String, Some(Literal::String(literal)))?;
            Ok(())
        } else {
            Err(UnterminatedSequence {
                span: self.span().into(),
                src: self.source.to_string(),
            }
            .into())
        }
    }

    fn scan_number_as_float(&mut self) -> Result<()> {
        let end = self.current;
        let mut precision = DEFAULT_FLOAT_PRECISION;

        if self.peek_is(|c| c == '_') {
            self.next(); // _
            while self.peek_is(|c| c.is_ascii_digit()) {
                self.next();
            }
            let literal = self.source[end + 1..self.current].to_string();
            precision = literal.parse().map_err(|_| MalformedFloatPrecision {
                span: Span::new(end, 1).into(),
                src: self.source.to_string(),
            })?;
        }

        let literal = self.source[self.start..end].to_string();
        let parse = Float::parse(literal);

        if let Ok(src) = parse {
            let mut float = Float::new(precision);
            float.assign(src);
            self.emit(TokenKind::Float, Some(Literal::Float(float)))
        } else {
            Err(MalformedNumber {
                span: self.span().into(),
                src: self.source.to_string(),
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
            self.emit(TokenKind::Integer, Some(Literal::Integer(integer)))
        } else {
            Err(MalformedNumber {
                span: self.span().into(),
                src: self.source.to_string(),
            }
            .into())
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
        } else if self.peek_is(|c| c == '_') {
            return self.scan_number_as_float();
        }

        return self.scan_number_as_integer();
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
            "and" => self.emit(TokenKind::And, None),
            "class" => self.emit(TokenKind::Class, None),
            "else" => self.emit(TokenKind::Else, None),
            "false" => self.emit(TokenKind::False, None),
            "fn" => self.emit(TokenKind::Fn, None),
            "for" => self.emit(TokenKind::For, None),
            "if" => self.emit(TokenKind::If, None),
            "nil" => self.emit(TokenKind::Nil, None),
            "or" => self.emit(TokenKind::Or, None),
            "print" => self.emit(TokenKind::Print, None),
            "return" => self.emit(TokenKind::Return, None),
            "super" => self.emit(TokenKind::Super, None),
            "this" => self.emit(TokenKind::This, None),
            "true" => self.emit(TokenKind::True, None),
            "var" => self.emit(TokenKind::Var, None),
            "while" => self.emit(TokenKind::While, None),
            other => self.emit(
                TokenKind::Identifier,
                Some(Literal::Identifier(other.to_string())),
            ),
        }
    }

    fn scan_token(&mut self, c: char) -> Result<()> {
        match c {
            '(' => self.emit(TokenKind::LeftParen, None),
            ')' => self.emit(TokenKind::RightParen, None),
            '{' => self.emit(TokenKind::LeftBrace, None),
            '}' => self.emit(TokenKind::RightBrace, None),
            ',' => self.emit(TokenKind::Comma, None),
            '.' => self.emit(TokenKind::Dot, None),
            '-' => self.emit(TokenKind::Minus, None),
            '+' => self.emit(TokenKind::Plus, None),
            ';' => self.emit(TokenKind::Semicolon, None),
            '*' => self.emit(TokenKind::Star, None),
            '/' => self.emit(TokenKind::Slash, None),
            '|' => self.emit(TokenKind::Pipe, None),
            '"' => self.scan_string(),
            '!' => {
                if self.matches('=') {
                    self.emit(TokenKind::BangEqual, None)
                } else {
                    self.emit(TokenKind::Bang, None)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.emit(TokenKind::EqualEqual, None)
                } else {
                    self.emit(TokenKind::Equal, None)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.emit(TokenKind::GreaterEqual, None)
                } else {
                    self.emit(TokenKind::Greater, None)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.emit(TokenKind::LessEqual, None)
                } else {
                    self.emit(TokenKind::Less, None)
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
                        span: self.span().into(),
                        src: self.source.to_string(),
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

        Ok(&self.tokens)
    }
}
