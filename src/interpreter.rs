use std::cell::RefCell;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::rc::Rc;

use miette::Report;

use crate::env::Env;
use crate::expr::Expr;
use crate::expr::ExprKind;
use crate::expr::ExprVisitor;
use crate::expr::Stmt;
use crate::expr::StmtKind;
use crate::expr::StmtVisitor;
use crate::token::TokenKind;
use crate::value::Value;

pub struct Interpreter<'a> {
    source: &'a str,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn interpret(
        &mut self,
        env: &Rc<RefCell<Env>>,
        statements: &Vec<Stmt>,
    ) -> Result<Value, Report> {
        match statements.len() {
            0 => Ok(Value::Nil),
            1 => self.visit_stmt(env, &statements[0]),
            _ => {
                if statements.len() > 1 {
                    for stmt in statements.iter().take(statements.len() - 1) {
                        self.visit_stmt(env, stmt)?;
                    }
                }

                self.visit_stmt(env, &statements[statements.len() - 1]) // keep the value of the last
            }
        }
    }
}

impl<'a> ExprVisitor<Value> for Interpreter<'a> {
    type Error = Report;

    fn visit_value(&mut self, _: &Rc<RefCell<Env>>, value: &Value) -> Result<Value, Self::Error> {
        Ok(value.clone())
    }

    fn visit_expr(&mut self, env: &Rc<RefCell<Env>>, expr: &Expr) -> Result<Value, Self::Error> {
        match &expr.kind {
            ExprKind::Grouping { expr } => self.visit_expr(env, expr),
            ExprKind::Literal { value } => self.visit_value(env, value),
            ExprKind::Unary { op, right } => {
                let value = self.visit_expr(env, right)?;
                match op.kind {
                    TokenKind::Minus => value
                        .neg()
                        .map_err(|e| e.into_report(&expr.span, self.source)),
                    TokenKind::Bang => Ok(!value),
                    _ => unreachable!(),
                }
            }
            ExprKind::Binary { left, op, right } => {
                let l = self.visit_expr(env, left)?;
                let r = self.visit_expr(env, right)?;
                match op.kind {
                    TokenKind::Minus => {
                        l.sub(r).map_err(|e| e.into_report(&expr.span, self.source))
                    }
                    TokenKind::Slash => {
                        l.div(r).map_err(|e| e.into_report(&expr.span, self.source))
                    }
                    TokenKind::Star => l.mul(r).map_err(|e| e.into_report(&expr.span, self.source)),
                    TokenKind::Plus => l.add(r).map_err(|e| e.into_report(&expr.span, self.source)),
                    TokenKind::Greater => Ok((l.gt(&r)).into()),
                    TokenKind::GreaterEqual => Ok((l.ge(&r)).into()),
                    TokenKind::Less => Ok((l.lt(&r)).into()),
                    TokenKind::LessEqual => Ok((l.le(&r)).into()),
                    TokenKind::EqualEqual => Ok((l.eq(&r)).into()),
                    TokenKind::BangEqual => Ok((!l.eq(&r)).into()),
                    _ => unreachable!(),
                }
            }
            ExprKind::Variable { name } => env
                .borrow_mut()
                .get(name)
                .map_err(|e| e.into_report(&expr.span, self.source)),
            ExprKind::Assign { name, expr } => {
                let value = self.visit_expr(env, expr)?;
                env.borrow_mut()
                    .assign(name.clone(), value.clone())
                    .map_err(|e| e.into_report(&expr.span, self.source))?;
                Ok(value)
            }
            ExprKind::Logical { left, op, right } => {
                let left = self.visit_expr(env, left)?;

                if op.kind == TokenKind::Or && left.is_truthy() {
                    Ok(left)
                } else if op.kind == TokenKind::And && !left.is_truthy() {
                    Ok(left)
                } else {
                    self.visit_expr(env, right)
                }
            }
        }
    }
}

impl<'a> StmtVisitor<Value> for Interpreter<'a> {
    type Error = Report;

    fn visit_stmt(&mut self, env: &Rc<RefCell<Env>>, stmt: &Stmt) -> Result<Value, Self::Error> {
        match &stmt.kind {
            StmtKind::Expression { expr } => self.visit_expr(env, expr),
            StmtKind::Print { expr } => {
                let value = self.visit_expr(env, expr)?;
                println!("{}", value);
                Ok(Value::Nil)
            }
            StmtKind::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.visit_expr(env, expr)?,
                    None => Value::Nil,
                };

                env.borrow_mut().define(name.clone(), value);

                Ok(Value::Nil)
            }
            StmtKind::Block { statements } => {
                let new_env = Rc::new(RefCell::new(Env::with_parent(env)));
                self.interpret(&new_env, statements)?;

                Ok(Value::Nil)
            }
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let value = self.visit_expr(env, cond)?;
                if value.is_truthy() {
                    self.visit_stmt(env, then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.visit_stmt(env, else_branch)
                } else {
                    Ok(Value::Nil)
                }
            }
            StmtKind::While { cond, body } => {
                while self.visit_expr(env, cond)?.is_truthy() {
                    self.visit_stmt(env, body)?;
                }

                Ok(Value::Nil)
            }
        }
    }
}
