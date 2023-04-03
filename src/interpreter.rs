use std::cell::RefCell;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::rc::Rc;

use miette::Report;

use crate::context::Ctx;
use crate::expr::Expr;
use crate::expr::ExprKind;
use crate::expr::Stmt;
use crate::expr::StmtKind;
use crate::function::Simple;
use crate::report::CalleeTypeError;
use crate::token::TokenKind;
use crate::value::Value;

pub enum RuntimeError {
    Report(Report),
    Return(Value),
}

impl From<Report> for RuntimeError {
    fn from(value: Report) -> Self {
        Self::Report(value)
    }
}

impl From<Value> for RuntimeError {
    fn from(value: Value) -> Self {
        Self::Return(value)
    }
}

pub fn interpret(ctx: &Rc<RefCell<Ctx>>, statements: &Vec<Stmt>) -> Result<Value, RuntimeError> {
    match statements.len() {
        0 => Ok(Value::Nil),
        1 => visit_stmt(ctx, &statements[0]),
        _ => {
            if statements.len() > 1 {
                for stmt in statements.iter().take(statements.len() - 1) {
                    visit_stmt(ctx, stmt)?;
                }
            }

            visit_stmt(ctx, &statements[statements.len() - 1]) // keep the value of the last
        }
    }
}

fn visit_value(_: &Rc<RefCell<Ctx>>, value: &Value) -> Result<Value, RuntimeError> {
    Ok(value.clone())
}

fn visit_expr(ctx: &Rc<RefCell<Ctx>>, expr: &Expr) -> Result<Value, RuntimeError> {
    match &expr.kind {
        ExprKind::Grouping { value } => visit_expr(ctx, value),
        ExprKind::Literal { value } => visit_value(ctx, value),
        ExprKind::Unary { op, right } => {
            let value = visit_expr(ctx, right)?;
            match op.kind {
                TokenKind::Minus => Ok(value.neg().map_err(|e| e.into_report(&expr.span))?),
                TokenKind::Bang => Ok(!value),
                _ => unreachable!(),
            }
        }
        ExprKind::Binary { left, op, right } => {
            let l = visit_expr(ctx, left)?;
            let r = visit_expr(ctx, right)?;
            match op.kind {
                TokenKind::Minus => Ok(l.sub(r).map_err(|e| e.into_report(&expr.span))?),
                TokenKind::Slash => Ok(l.div(r).map_err(|e| e.into_report(&expr.span))?),
                TokenKind::Star => Ok(l.mul(r).map_err(|e| e.into_report(&expr.span))?),
                TokenKind::Plus => Ok(l.add(r).map_err(|e| e.into_report(&expr.span))?),
                TokenKind::Greater => Ok((l.gt(&r)).into()),
                TokenKind::GreaterEqual => Ok((l.ge(&r)).into()),
                TokenKind::Less => Ok((l.lt(&r)).into()),
                TokenKind::LessEqual => Ok((l.le(&r)).into()),
                TokenKind::EqualEqual => Ok((l.eq(&r)).into()),
                TokenKind::BangEqual => Ok((!l.eq(&r)).into()),
                _ => unreachable!(),
            }
        }
        ExprKind::Variable { name } => Ok(ctx
            .borrow()
            .get(expr, name)
            .map_err(|e| e.into_report(&expr.span))?),
        ExprKind::Assign { name, value } => {
            let value = visit_expr(ctx, value)?;
            ctx.borrow_mut()
                .assign(expr, name.clone(), value.clone())
                .map_err(|e| e.into_report(&expr.span))?;
            Ok(value)
        }
        ExprKind::Logical { left, op, right } => {
            let left = visit_expr(ctx, left)?;

            if (op.kind == TokenKind::Or && left.is_truthy())
                || (op.kind == TokenKind::And && !left.is_truthy())
            {
                Ok(left)
            } else {
                visit_expr(ctx, right)
            }
        }
        ExprKind::Call { callee, args } => {
            let callee = visit_expr(ctx, callee)?;
            let args = args
                .iter()
                .map(|e| visit_expr(ctx, e))
                .collect::<Result<Vec<_>, _>>()?;

            match callee {
                Value::Function(f) => Ok(f.call(ctx, args)?),
                _ => Err(RuntimeError::Report(
                    CalleeTypeError {
                        span: expr.span.into(),
                    }
                    .into(),
                )),
            }
        }
    }
}

fn visit_stmt(ctx: &Rc<RefCell<Ctx>>, stmt: &Stmt) -> Result<Value, RuntimeError> {
    match &stmt.kind {
        StmtKind::Expression { expr } => visit_expr(ctx, expr),
        StmtKind::Let { name, initializer } => {
            let value = match initializer {
                Some(expr) => visit_expr(ctx, expr)?,
                None => Value::Nil,
            };

            ctx.borrow_mut().define(name.clone(), value);

            Ok(Value::Nil)
        }
        StmtKind::Block { statements } => {
            let new_env = Ctx::with_parent(ctx);
            interpret(&new_env, statements)?;

            Ok(Value::Nil)
        }
        StmtKind::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let value = visit_expr(ctx, cond)?;
            if value.is_truthy() {
                visit_stmt(ctx, then_branch)
            } else if let Some(else_branch) = else_branch {
                visit_stmt(ctx, else_branch)
            } else {
                Ok(Value::Nil)
            }
        }
        StmtKind::While { cond, body } => {
            while visit_expr(ctx, cond)?.is_truthy() {
                visit_stmt(ctx, body)?;
            }

            Ok(Value::Nil)
        }
        StmtKind::Function { name, params, body } => {
            let function = Value::Function(Rc::new(Simple {
                params: params.clone(),
                body: body.clone(),
                closure: ctx.clone(),
            }));

            ctx.borrow_mut().define(name.clone(), function);

            Ok(Value::Nil)
        }
        StmtKind::Return { expr } => {
            let value = match expr {
                Some(expr) => visit_expr(ctx, expr)?,
                None => Value::Nil,
            };

            Err(RuntimeError::Return(value))
        }
    }
}
