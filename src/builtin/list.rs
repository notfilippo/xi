use super::builtin;
use crate::{token::Literal, value::Value};

builtin!(LenBuiltin, "len", 1, _ctx, args, {
    if let Some(item) = args.first() {
        match item {
            Value::List(list) => Ok(Value::Literal(Literal::Integer(
                list.borrow().items.len().into(),
            ))),
            _ => Ok(Value::Literal(Literal::Integer(0.into()))),
        }
    } else {
        Ok(Value::Nil)
    }
});
