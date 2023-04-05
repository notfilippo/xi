use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use miette::Report;
use thiserror::Error;

use crate::{builtin::*, report::UndefinedValue, resolver::Resolver, token::Span, value::Value};

#[derive(Default, Clone, Debug)]
pub struct Env {
    values: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Env>>>,
    resolver: Rc<RefCell<Resolver>>,
}

impl Env {
    pub fn with_parent(enclosing: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::default(),
            resolver: enclosing.borrow().resolver.clone(),
            enclosing: Some(enclosing.clone()),
        }))
    }

    pub fn global() -> Rc<RefCell<Self>> {
        let mut global = Self::default();
        global.define("time", Value::Function(Rc::new(TimeBuiltin {})));
        global.define("print", Value::Function(Rc::new(PrintBuiltin {})));
        global.define("println", Value::Function(Rc::new(PrintlnBuiltin {})));
        global.define("len", Value::Function(Rc::new(LenBuiltin {})));
        Rc::new(RefCell::new(global))
    }
}

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("undefined variable")]
    UndefinedValue,
}

impl EnvError {
    pub fn into_report(self, span: &Span) -> Report {
        match self {
            EnvError::UndefinedValue => UndefinedValue {
                span: (*span).into(),
            }
            .into(),
        }
    }
}

impl Env {
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, distance: usize, name: &str, value: Value) -> Result<(), EnvError> {
        if distance == 0 {
            if let Entry::Occupied(mut e) = self.values.entry(name.to_string()) {
                e.insert(value);
                return Ok(());
            }
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(distance - 1, name, value)
        } else {
            Err(EnvError::UndefinedValue)
        }
    }

    pub fn get(&self, distance: usize, name: &str) -> Result<Value, EnvError> {
        if distance == 0 {
            if let Some(value) = self.values.get(name) {
                return Ok(value.clone());
            }
        }

        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(distance - 1, name)
        } else {
            Err(EnvError::UndefinedValue)
        }
    }
}
