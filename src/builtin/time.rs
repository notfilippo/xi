use std::time::{SystemTime, UNIX_EPOCH};

use super::builtin;
use crate::{token::Literal, value::Value};

builtin!(TimeBuiltin, "time", 0, _ctx, _args, {
    let time = SystemTime::now();
    let epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    Ok(Value::Literal(Literal::Integer(epoch.as_nanos().into())))
});
