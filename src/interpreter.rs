use crate::expr::Expr;
use crate::expr::Visitor;
use crate::token::TokenKind;
use crate::value::{Error, Value};

pub struct Interpreter<'a> {
    source: &'a str,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

impl<'a> Visitor<Value> for Interpreter<'a> {
    type Error = Error;

    fn visit_value(&mut self, value: &Value) -> Result<Value, Self::Error> {
        Ok(value.clone())
    }

    fn visit_expr(&mut self, expr: &Box<Expr>) -> Result<Value, Self::Error> {
        match &**expr {
            Expr::Grouping { expr } => self.visit_expr(&expr),
            Expr::Literal { value } => self.visit_value(&value),
            Expr::Unary { op, right } => {
                let value = self.visit_expr(&right)?;
                match op.kind {
                    TokenKind::Minus => -value,
                    TokenKind::Bang => Ok(!value),
                    _ => unreachable!(),
                }
            }
            Expr::Binary { left, op, right } => {
                let l = self.visit_expr(&left)?;
                let r = self.visit_expr(&right)?;
                match op.kind {
                    TokenKind::Minus => l - r,
                    TokenKind::Slash => l / r,
                    TokenKind::Star => l * r,
                    TokenKind::Plus => l + r,
                    TokenKind::Greater => Ok((l > r).into()),
                    TokenKind::GreaterEqual => Ok((l >= r).into()),
                    TokenKind::Less => Ok((l < r).into()),
                    TokenKind::LessEqual => Ok((l <= r).into()),
                    TokenKind::EqualEqual => Ok((l == r).into()),
                    TokenKind::BangEqual => Ok((l != r).into()),
                    _ => unreachable!(),
                }
            }
        }
    }
}
