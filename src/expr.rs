use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::token::{Span, Token};
use crate::value::Value;

#[derive(Debug)]
pub enum ExprKind {
    Assign {
        name: String,
        expr: Box<Expr>,
    },
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
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Variable {
        name: String,
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expr: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expr: Box<Expr>,
    },
    Var {
        name: String,
        initializer: Option<Box<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Stmt>,
    },
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

pub trait ExprVisitor<T> {
    type Error;

    fn visit_value(&mut self, env: &Rc<RefCell<Env>>, value: &Value) -> Result<T, Self::Error>;
    fn visit_expr(&mut self, env: &Rc<RefCell<Env>>, expr: &Expr) -> Result<T, Self::Error>;
}

pub trait StmtVisitor<T> {
    type Error;

    fn visit_stmt(&mut self, env: &Rc<RefCell<Env>>, stmt: &Stmt) -> Result<T, Self::Error>;
}
