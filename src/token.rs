use std::rc::Rc;

use miette::SourceSpan;

#[derive(Default, Debug)]
pub struct Span {
    offset: usize,
    length: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self.offset.into(), self.length.into())
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Pipe,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Integer(rug::Integer),
    Float(rug::Float),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
