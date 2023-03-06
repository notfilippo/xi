use crate::token::{Span, Token};
use crate::value::Value;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
        span: Span,
    },
    Grouping {
        expr: Box<Expr>,
        span: Span,
    },
    Literal {
        value: Value,
        span: Span,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
        span: Span,
    },
}

pub trait Visitor<T> {
    type Error;

    fn visit_value(&mut self, value: &Value) -> Result<T, Self::Error>;
    fn visit_expr(&mut self, expr: &Box<Expr>) -> Result<T, Self::Error>;
}
