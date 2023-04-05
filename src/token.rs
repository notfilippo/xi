use std::hash::Hash;

use miette::SourceSpan;

#[derive(Default, Debug, Clone, Copy, Hash)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }

    pub fn new_range(start: usize, end: usize) -> Self {
        Self {
            offset: start,
            length: end - start,
        }
    }
}

impl From<Span> for SourceSpan {
    fn from(val: Span) -> Self {
        SourceSpan::new(val.offset.into(), val.length.into())
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    String(String),
    Integer(rug::Integer),
    Float(rug::Float),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<Literal>,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, literal: Option<Literal>, span: Span) -> Self {
        Self {
            kind,
            literal,
            span,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
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
    Identifier,
    String,
    Integer,
    Float,

    // Keywords.
    And,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Return,
    True,
    Let,
    While,
}
