use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    context::Ctx,
    expr::Stmt,
    interpreter::{interpret, RuntimeError},
    token::Literal,
    value::Value,
};

pub trait Function: std::fmt::Debug {
    fn call(&self, env: &Rc<RefCell<Ctx>>, args: Vec<Value>) -> Result<Value, RuntimeError> {
        match self.run(env, args) {
            Ok(value) => Ok(value),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(err) => Err(err),
        }
    }

    fn run(&self, env: &Rc<RefCell<Ctx>>, args: Vec<Value>) -> Result<Value, RuntimeError>;
    fn arity(&self) -> usize;
}

#[derive(Debug)]
pub struct Simple {
    pub params: Rc<Vec<String>>,
    pub body: Rc<Vec<Stmt>>,
    pub closure: Rc<RefCell<Ctx>>,
}

impl Function for Simple {
    fn run(&self, _: &Rc<RefCell<Ctx>>, args: Vec<Value>) -> Result<Value, RuntimeError> {
        let new_env = Ctx::with_parent(&self.closure);

        for (i, name) in self.params.iter().enumerate() {
            new_env.borrow_mut().define(name.clone(), args[i].clone())
        }

        interpret(&new_env, &self.body)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

#[derive(Debug)]
pub struct TimeBuiltin;

impl Function for TimeBuiltin {
    fn run(&self, _: &Rc<RefCell<Ctx>>, _: Vec<Value>) -> Result<Value, RuntimeError> {
        let time = SystemTime::now();
        let epoch = time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(Value::Literal(Literal::Integer(epoch.as_nanos().into())))
    }

    fn arity(&self) -> usize {
        0
    }
}
