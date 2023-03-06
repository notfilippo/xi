use miette::SourceSpan;

#[derive(Default, Debug, Clone, Copy)]
pub struct Span {
    offset: usize,
    length: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }

    pub fn new_range(start: usize, end: usize) -> Self {
        Self { offset: start, length: end - start }
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
    Identifier,
    String,
    Integer,
    Float,

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
}
