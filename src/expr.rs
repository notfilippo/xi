use crate::token::Token;
use crate::value::Value;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
}

pub trait Visitor<T> {
    type Error;

    fn visit_value(&mut self, value: &Value) -> Result<T, Self::Error>;
    fn visit_expr(&mut self, expr: &Box<Expr>) -> Result<T, Self::Error>;
}
