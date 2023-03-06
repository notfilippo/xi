use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use miette::Report;

use crate::expr::Expr;
use crate::expr::Visitor;
use crate::token::TokenKind;
use crate::value::Value;

pub struct Interpreter<'a> {
    source: &'a str,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

impl<'a> Visitor<Value> for Interpreter<'a> {
    type Error = Report;

    fn visit_value(&mut self, value: &Value) -> Result<Value, Self::Error> {
        Ok(value.clone())
    }

    fn visit_expr(&mut self, expr: &Box<Expr>) -> Result<Value, Self::Error> {
        match &**expr {
            Expr::Grouping { expr, span: _ } => self.visit_expr(&expr),
            Expr::Literal { value, span: _ } => self.visit_value(&value),
            Expr::Unary { op, right, span } => {
                let value = self.visit_expr(&right)?;
                match op.kind {
                    TokenKind::Minus => {
                        (-value).map_err(|e| e.into_report(span, self.source))
                    }
                    TokenKind::Bang => Ok(!value),
                    _ => unreachable!(),
                }
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let l = self.visit_expr(&left)?;
                let r = self.visit_expr(&right)?;
                match op.kind {
                    TokenKind::Minus => l.sub(r).map_err(|e| e.into_report(span, self.source)),
                    TokenKind::Slash => l.div(r).map_err(|e| e.into_report(span, self.source)),
                    TokenKind::Star => l.mul(r).map_err(|e| e.into_report(span, self.source)),
                    TokenKind::Plus => l.add(r).map_err(|e| e.into_report(span, self.source)),
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
