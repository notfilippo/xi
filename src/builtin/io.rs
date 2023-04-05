use super::builtin;
use crate::value::Value;

builtin!(PrintBuiltin, "print", 0, _ctx, args, {
    let strings = args.into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
    print!("{}", strings.join(" "));

    Ok(Value::Nil)
});

builtin!(PrintlnBuiltin, "println", 0, _ctx, args, {
    let strings = args.into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
    println!("{}", strings.join(" "));

    Ok(Value::Nil)
});
