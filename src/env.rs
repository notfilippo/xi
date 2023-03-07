use std::{collections::{hash_map::Entry, HashMap}, cell::RefCell, rc::Rc};

use miette::Report;
use thiserror::Error;

use crate::{report::UndefinedValue, token::Span, value::Value, builtin::TimeBuiltin};

#[derive(Default, Clone)]
pub struct Env {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn with_parent(enclosing: &Rc<RefCell<Env>>) -> Self {
        Self {
            values: HashMap::default(),
            enclosing: Some(enclosing.clone()),
        }
    }

    pub fn global() -> Self {
        let mut global = Env::default();
        global.define("time".to_string(), Value::Function(Rc::new(TimeBuiltin {})));
        global
    }
}

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("undefined variable")]
    UndefinedValue,
}

impl EnvError {
    pub fn into_report(self, span: &Span, source: &str) -> Report {
        match self {
            EnvError::UndefinedValue => UndefinedValue {
                span: (*span).into(),
                src: source.to_string(),
            }
            .into(),
        }
    }
}

impl Env {
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), EnvError> {
        if let Entry::Occupied(mut e) = self.values.entry(name.clone()) {
            e.insert(value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(EnvError::UndefinedValue)
        }
    }

    pub fn get(&self, name: &String) -> Result<Value, EnvError> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(EnvError::UndefinedValue)
        }
    }
}
