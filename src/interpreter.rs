use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::rc::Rc;

use miette::Report;

use crate::context::Ctx;
use crate::dict::Dict;
use crate::expr::Expr;
use crate::expr::ExprKind;
use crate::expr::Stmt;
use crate::expr::StmtKind;
use crate::function::SimpleFunction;
use crate::list::List;
use crate::report::CalleeTypeError;
use crate::report::IndexTypeError;
use crate::report::InstanceTypeError;
use crate::report::ListIndexInvalidError;
use crate::report::ListIndexOutOfBoundsError;
use crate::token::Literal;
use crate::token::TokenKind;
use crate::value::Value;
use crate::value::ValueKey;

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
                .assign(expr, name, value.clone())
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
        ExprKind::Get { obj, name: _ } => {
            let _this = visit_expr(ctx, obj)?;
            Err(RuntimeError::Report(
                InstanceTypeError {
                    span: expr.span.into(),
                }
                .into(),
            ))
        }
        ExprKind::Set {
            obj,
            name: _,
            value,
        } => {
            let _this = visit_expr(ctx, obj)?;
            let _value = visit_expr(ctx, value)?;
            Err(RuntimeError::Report(
                InstanceTypeError {
                    span: expr.span.into(),
                }
                .into(),
            ))
        }
        ExprKind::List { items } => {
            let items = items
                .iter()
                .map(|expr| visit_expr(ctx, expr))
                .collect::<Result<Vec<_>, _>>()?;

            let list = List(items);
            Ok(Value::List(Rc::new(RefCell::new(list))))
        }
        ExprKind::Dict { items } => {
            let mut i = HashMap::new();
            for (left, right) in items {
                let left = visit_expr(ctx, left)?;
                let right = visit_expr(ctx, right)?;
                i.insert(ValueKey(left), right);
            }

            let list = Dict(i);
            Ok(Value::Dict(Rc::new(RefCell::new(list))))
        }
        ExprKind::GetIndex { obj, index } => {
            let this = visit_expr(ctx, obj)?;
            match this {
                Value::List(list) => {
                    let index = visit_expr(ctx, index)?;
                    match index {
                        Value::Literal(Literal::Integer(i)) => {
                            let index: usize = i.try_into().map_err(|_| {
                                RuntimeError::Report(
                                    ListIndexInvalidError {
                                        span: expr.span.into(),
                                    }
                                    .into(),
                                )
                            })?;
                            match list.borrow().0.get(index) {
                                Some(value) => Ok(value.clone()),
                                None => Err(RuntimeError::Report(
                                    ListIndexOutOfBoundsError {
                                        span: expr.span.into(),
                                    }
                                    .into(),
                                )),
                            }
                        }
                        _ => Err(RuntimeError::Report(
                            ListIndexInvalidError {
                                span: expr.span.into(),
                            }
                            .into(),
                        )),
                    }
                }
                Value::Dict(dict) => {
                    let index = ValueKey(visit_expr(ctx, index)?);
                    Ok(dict.borrow().0.get(&index).unwrap().clone())
                }
                _ => Err(RuntimeError::Report(
                    IndexTypeError {
                        span: expr.span.into(),
                    }
                    .into(),
                )),
            }
        }
        ExprKind::SetIndex { obj, index, value } => {
            let this = visit_expr(ctx, obj)?;
            match this {
                Value::List(list) => {
                    let index = visit_expr(ctx, index)?;
                    match index {
                        Value::Literal(Literal::Integer(i)) => {
                            let index: usize = i.try_into().map_err(|_| {
                                RuntimeError::Report(
                                    ListIndexInvalidError {
                                        span: expr.span.into(),
                                    }
                                    .into(),
                                )
                            })?;
                            match list.borrow_mut().0.get_mut(index) {
                                Some(prev) => {
                                    let new = visit_expr(ctx, value)?;
                                    *prev = new.clone();
                                    Ok(new)
                                }
                                None => Err(RuntimeError::Report(
                                    ListIndexOutOfBoundsError {
                                        span: expr.span.into(),
                                    }
                                    .into(),
                                )),
                            }
                        }
                        _ => Err(RuntimeError::Report(
                            ListIndexInvalidError {
                                span: expr.span.into(),
                            }
                            .into(),
                        )),
                    }
                }
                Value::Dict(dict) => {
                    let value = visit_expr(ctx, value)?;
                    let index = ValueKey(visit_expr(ctx, index)?);
                    dict.borrow_mut().0.insert(index, value.clone());
                    Ok(value)
                }
                _ => Err(RuntimeError::Report(
                    IndexTypeError {
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

            ctx.borrow_mut().define(name, value);

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
            let function = SimpleFunction {
                name: name.clone(),
                params: params.clone(),
                body: body.clone(),
                closure: ctx.clone(),
            };

            ctx.borrow_mut()
                .define(name, Value::Function(Rc::new(function)));

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
