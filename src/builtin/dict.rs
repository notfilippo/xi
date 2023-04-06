use std::{cell::RefCell, rc::Rc};

use super::builtin;
use crate::{list::List, token::Literal, value::Value};

builtin!(KeysBuiltin, "keys", 1, _ctx, args, {
    if let Some(item) = args.first() {
        match item {
            Value::Dict(value) => {
                let keys = value
                    .borrow()
                    .0
                    .keys()
                    .into_iter()
                    .map(|key| key.0.clone())
                    .collect();
                Ok(Value::List(Rc::new(RefCell::new(List(keys)))))
            }
            _ => Ok(Value::Literal(Literal::Integer(0.into()))),
        }
    } else {
        Ok(Value::Nil)
    }
});

builtin!(ValuesBuiltin, "values", 1, _ctx, args, {
    if let Some(item) = args.first() {
        match item {
            Value::Dict(value) => {
                let keys = value
                    .borrow()
                    .0
                    .values()
                    .into_iter()
                    .map(|value| value.clone())
                    .collect();
                Ok(Value::List(Rc::new(RefCell::new(List(keys)))))
            }
            _ => Ok(Value::Literal(Literal::Integer(0.into()))),
        }
    } else {
        Ok(Value::Nil)
    }
});
