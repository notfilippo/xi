use std::{cell::RefCell, rc::Rc};

use crate::{
    env::{Env, EnvError},
    expr::Expr,
    resolver::Resolver,
    value::Value,
};

#[derive(Debug)]
pub struct Ctx {
    env: Rc<RefCell<Env>>,
    resolver: Rc<Resolver>,
    size: usize,
}

impl Ctx {
    pub fn new(env: &Rc<RefCell<Env>>, resolver: Rc<Resolver>) -> Self {
        Self {
            env: env.clone(),
            resolver,
            size: 0,
        }
    }

    pub fn with_parent(ctx: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            env: Env::with_parent(&ctx.borrow().env),
            resolver: ctx.borrow().resolver.clone(),
            size: ctx.borrow().size + 1,
        }))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.env.borrow_mut().define(name, value);
    }

    pub fn assign(&mut self, expr: &Expr, name: String, value: Value) -> Result<(), EnvError> {
        let distance = *self.resolver.locals.get(&expr.id).unwrap_or(&self.size); // TODO: ??
        self.env.borrow_mut().assign(distance, name, value)
    }

    pub fn get(&self, expr: &Expr, name: &String) -> Result<Value, EnvError> {
        let distance = *self.resolver.locals.get(&expr.id).unwrap_or(&self.size); // TODO: ??
        self.env.borrow().get(distance, name)
    }
}
