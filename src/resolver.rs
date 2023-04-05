use std::collections::HashMap;

use miette::Report;

use crate::{
    expr::{Expr, ExprKind, Stmt, StmtKind},
    report::ReadLocalVariableInOwnInitializer,
};

#[derive(Default, Debug)]
pub struct Resolver {
    pub scopes: Vec<HashMap<String, bool>>,
    pub locals: HashMap<usize, usize>,
}

impl Resolver {
    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<(), Report> {
        if statements.len() > 1 {
            for stmt in statements {
                self.visit_stmt(stmt)?;
            }
        }

        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Default::default())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, string: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(string.to_string(), true);
        }
    }

    fn declare(&mut self, string: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(string.to_string(), false);
        }
    }

    fn resolve_local(&mut self, id: usize, name: &str) {
        for (depth, s) in self.scopes.iter().rev().enumerate() {
            if s.contains_key(name) {
                self.locals.insert(id, depth);
                return;
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<(), Report> {
        match &expr.kind {
            ExprKind::Variable { name } => {
                if let Some(scope) = self.scopes.last() {
                    if !scope.get(name).unwrap_or(&true) {
                        // check if it exists and it its set at false
                        return Err(ReadLocalVariableInOwnInitializer {
                            span: expr.span.into(),
                        }
                        .into());
                    }
                }

                self.resolve_local(expr.id, name)
            }
            ExprKind::Assign { name, value } => {
                self.visit_expr(value)?;
                self.resolve_local(expr.id, name)
            }
            ExprKind::Binary { left, op: _, right } => {
                self.visit_expr(left)?;
                self.visit_expr(right)?;
            }
            ExprKind::Call { callee, args } => {
                self.visit_expr(callee)?;
                for arg in args {
                    self.visit_expr(arg)?;
                }
            }
            ExprKind::Grouping { value } => {
                self.visit_expr(value)?;
            }
            ExprKind::Literal { value: _ } => {}
            ExprKind::Logical { left, op: _, right } => {
                self.visit_expr(left)?;
                self.visit_expr(right)?;
            }
            ExprKind::Unary { op: _, right } => {
                self.visit_expr(right)?;
            }
            ExprKind::Get { obj, name: _ } => {
                self.visit_expr(obj)?;
            }
            ExprKind::Set {
                obj,
                name: _,
                value,
            } => {
                self.visit_expr(obj)?;
                self.visit_expr(value)?;
            }
            ExprKind::List { items } => {
                for item in items {
                    self.visit_expr(item)?;
                }
            }
            ExprKind::GetIndex { obj, index } => {
                self.visit_expr(obj)?;
                self.visit_expr(index)?;
            },
            ExprKind::SetIndex { obj, index, value } => {
                self.visit_expr(obj)?;
                self.visit_expr(index)?;
                self.visit_expr(value)?;
            },
            ExprKind::Dict { items } => {
                for (left, right) in items {
                    self.visit_expr(left)?;
                    self.visit_expr(right)?;
                }
            },
        }

        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), Report> {
        match &stmt.kind {
            StmtKind::Block { statements } => {
                self.begin_scope();
                for statement in statements {
                    self.visit_stmt(statement)?;
                }
                self.end_scope()
            }
            StmtKind::Let { name, initializer } => {
                self.declare(name);
                if let Some(initializer) = initializer {
                    self.visit_expr(initializer)?
                }
                self.define(name);
            }
            StmtKind::Function { name, params, body } => {
                self.declare(name);
                self.define(name);

                self.begin_scope();
                for param in params.iter() {
                    self.declare(param);
                    self.define(param);
                }
                for statement in body.iter() {
                    self.visit_stmt(statement)?;
                }
                self.end_scope();
            }
            StmtKind::Expression { expr } => self.visit_expr(expr)?,
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.visit_expr(cond)?;
                self.visit_stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.visit_stmt(else_branch)?;
                }
            }
            StmtKind::Return { expr } => {
                if let Some(expr) = expr {
                    self.visit_expr(expr)?;
                }
            }
            StmtKind::While { cond, body } => {
                self.visit_expr(cond)?;
                self.visit_stmt(body)?;
            }
        }

        Ok(())
    }
}
