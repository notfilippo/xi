use std::{cell::RefCell, rc::Rc};

use crate::{
    env::{Env, EnvError},
    expr::Identifiable,
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

    pub fn define(&mut self, name: &str, value: Value) {
        self.env.borrow_mut().define(name, value);
    }

    pub fn assign<I: Identifiable>(
        &mut self,
        i: &I,
        name: &str,
        value: Value,
    ) -> Result<(), EnvError> {
        let distance = *self.resolver.locals.get(i.id()).unwrap_or(&self.size); // TODO: ??
        self.env.borrow_mut().assign(distance, name, value)
    }

    pub fn get<I: Identifiable>(&self, i: &I, name: &str) -> Result<Value, EnvError> {
        let distance = *self.resolver.locals.get(i.id()).unwrap_or(&self.size); // TODO: ??
        self.env.borrow().get(distance, name)
    }
}
