use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use miette::Report;

use crate::{
    env::Env,
    token::Literal,
    value::{Function, Value},
};

#[derive(Debug)]
pub struct TimeBuiltin;

impl Function for TimeBuiltin {
    fn run(&self, _: &Rc<RefCell<Env>>, _: Vec<Value>) -> Result<Value, Report> {
        let time = SystemTime::now();
        let epoch = time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(Value::Literal(Literal::Integer(epoch.as_nanos().into())))
    }
}
