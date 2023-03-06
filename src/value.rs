use std::{ops::{Add, Div, Mul, Neg, Not, Sub}, fmt::{Display, write}};

use rug::{Float, Integer};

use crate::token::Literal;

#[derive(Debug, Clone)]
pub enum Value {
    True,
    False,
    Nil,
    Literal(Literal),
}

pub enum Error {
    UnsupportedOperation,
}

impl Value {
    // this follows Ruby’s simple rule:
    // - false and nil are falsey
    // - everything else is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::False => false,
            Value::Nil => false,
            _ => true,
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        self.is_truthy().not().into()
    }
}

impl Neg for Literal {
    type Output = Result<Self, Error>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(i) => Ok(i.clone().neg().into()),
            Self::Float(f) => Ok(f.clone().neg().into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Neg for Value {
    type Output = Result<Self, Error>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Literal(literal) => Ok(literal.neg()?.into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Add for Literal {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.add(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.add(rhs).into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Add for Value {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.add(rhs)?.into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.sub(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.sub(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.sub(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.sub(rhs).into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.sub(rhs)?.into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Div for Literal {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.div(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.div(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.div(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.div(rhs).into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Div for Value {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.div(rhs)?.into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::Integer(lhs), Self::Float(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::Float(lhs), Self::Integer(rhs)) => Ok(lhs.mul(rhs).into()),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(lhs.mul(rhs).into()),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Literal(lhs), Self::Literal(rhs)) => Ok(lhs.mul(rhs)?.into()),
            _ => Err(Error::UnsupportedOperation),
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
            _ => None
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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => Value::True,
            false => Value::False,
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
