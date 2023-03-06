use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use miette::Report;
use rug::{integer::TryFromIntegerError, Float, Integer};
use thiserror::Error;

use crate::{
    report::UnsupportedOperation,
    token::{Literal, Span},
};

#[derive(Debug, Clone)]
pub enum Value {
    True,
    False,
    Nil,
    Literal(Literal),
}

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("unsupported operation")]
    UnsupportedOperation,
    #[error("data store disconnected")]
    IntegerConversionError(#[from] TryFromIntegerError),
}

impl Value {
    // this follows Ruby’s simple rule:
    // - false and nil are falsey
    // - everything else is truthy
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::False | Value::Nil)
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        self.is_truthy().not().into()
    }
}

impl Neg for Literal {
    type Output = Result<Self, ValueError>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(i) => Ok(i.neg().into()),
            Self::Float(f) => Ok(f.neg().into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Neg for Value {
    type Output = Result<Self, ValueError>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Literal(literal) => Ok(literal.neg()?.into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Add for Literal {
    type Output = Result<Self, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::String(lhs), rhs) => Ok(format!("{}{}", lhs, rhs).into()),
            (lhs, Self::String(rhs)) => Ok(format!("{}{}", lhs, rhs).into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Add for Value {
    type Output = Result<Self, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.add(rhs)?.into()),
            (Self::Literal(Literal::String(lhs)), rhs) => Ok(format!("{}{}", lhs, rhs).into()),
            (lhs, Self::Literal(Literal::String(rhs))) => Ok(format!("{}{}", lhs, rhs).into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Self, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.sub(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.sub(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.sub(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.sub(rhs).into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.sub(rhs)?.into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Div for Literal {
    type Output = Result<Self, ValueError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.div(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.div(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.div(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.div(rhs).into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Div for Value {
    type Output = Result<Self, ValueError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.div(rhs)?.into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Self, ValueError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::String(lhs), Self::Integer(rhs)) => Ok(lhs.repeat(rhs.try_into()?).into()),
            (Self::Integer(lhs), Self::String(rhs)) => Ok(rhs.repeat(lhs.try_into()?).into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, ValueError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.mul(rhs)?.into()),
            _ => Err(ValueError::UnsupportedOperation),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => lhs.partial_cmp(rhs),
            (Self::Integer(lhs), Self::Float(rhs)) => lhs.partial_cmp(rhs),
            (Self::Float(lhs), Self::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Self::Integer(lhs), Self::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Self::String(lhs), Self::String(rhs)) => lhs.partial_cmp(rhs),
            (Self::Identifier(lhs), Self::Identifier(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => lhs.partial_cmp(rhs),
            (Self::True, Self::False) => true.partial_cmp(&false),
            (Self::False, Self::True) => false.partial_cmp(&true),
            _ => None,
        }
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<Integer> for Literal {
    fn from(integer: Integer) -> Self {
        Self::Integer(integer)
    }
}

impl From<Integer> for Value {
    fn from(integer: Integer) -> Self {
        Self::Literal(integer.into())
    }
}

impl From<Float> for Literal {
    fn from(float: Float) -> Self {
        Self::Float(float)
    }
}

impl From<Float> for Value {
    fn from(float: Float) -> Self {
        Self::Literal(float.into())
    }
}

impl From<&str> for Literal {
    fn from(string: &str) -> Self {
        Self::String(string.to_string())
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Self::Literal(string.into())
    }
}

impl From<String> for Literal {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Self::Literal(string.into())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => Value::True,
            false => Value::False,
        }
    }
}

impl ValueError {
    pub fn into_report(self, span: &Span, source: &str) -> Report {
        match self {
            ValueError::UnsupportedOperation => UnsupportedOperation {
                span: (*span).into(),
                src: source.to_string(),
            }
            .into(),
            ValueError::IntegerConversionError(_) => UnsupportedOperation {
                span: (*span).into(),
                src: source.to_string(),
            }
            .into(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(value) => value.fmt(f),
            Self::String(value) => value.fmt(f),
            Self::Integer(value) => value.fmt(f),
            Self::Float(value) => value.fmt(f),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
            Self::Literal(value) => value.fmt(f),
        }
    }
}
