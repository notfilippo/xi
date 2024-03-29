use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::token::{Span, Token};
use crate::value::Value;

pub trait Identifiable {
    fn id(&self) -> &usize;
}

#[derive(Debug)]
pub enum ExprKind {
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    GetIndex {
        obj: Box<Expr>,
        index: Box<Expr>,
    },
    SetIndex {
        obj: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    List {
        items: Vec<Expr>,
    },
    Dict {
        items: Vec<(Expr, Expr)>,
    },
    Get {
        obj: Box<Expr>,
        name: String,
    },
    Grouping {
        value: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Set {
        obj: Box<Expr>,
        name: String,
        value: Box<Expr>,
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
    pub id: usize,
}

impl Identifiable for Expr {
    fn id(&self) -> &usize {
        &self.id
    }
}

#[derive(Debug)]
pub enum StmtKind {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expr: Box<Expr>,
    },
    Function {
        name: String,
        params: Rc<Vec<String>>,
        body: Rc<Vec<Stmt>>,
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Return {
        expr: Option<Box<Expr>>,
    },
    Let {
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
    pub id: usize,
}

impl Identifiable for Stmt {
    fn id(&self) -> &usize {
        &self.id
    }
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
