use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
};

use crate::{
    context::Ctx,
    expr::Stmt,
    interpreter::{interpret, RuntimeError},
    value::Value,
};

pub trait Function: std::fmt::Debug + std::fmt::Display {
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

#[derive(Debug, Clone)]
pub struct SimpleFunction {
    pub name: String,
    pub params: Rc<Vec<String>>,
    pub body: Rc<Vec<Stmt>>,
    pub closure: Rc<RefCell<Ctx>>,
}

impl From<SimpleFunction> for Value {
    fn from(val: SimpleFunction) -> Self {
        Value::Function(Rc::new(val))
    }
}

impl Display for SimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.name)
    }
}

impl Function for SimpleFunction {
    fn run(&self, _: &Rc<RefCell<Ctx>>, args: Vec<Value>) -> Result<Value, RuntimeError> {
        let new_env = Ctx::with_parent(&self.closure);

        for (i, name) in self.params.iter().enumerate() {
            new_env.borrow_mut().define(name, args[i].clone())
        }

        interpret(&new_env, &self.body)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
