mod dict;
mod io;
mod seq;
mod time;

pub use dict::*;
pub use io::*;
pub use seq::*;
pub use time::*;

macro_rules! builtin {
    ($name:ident, $fn_name:expr, $arity:expr, $ctx:ident, $args:ident, $body:expr) => {
        #[derive(Debug)]
        pub struct $name;

        impl crate::function::Function for $name {
            fn run(
                &self,
                $ctx: &std::rc::Rc<std::cell::RefCell<crate::context::Ctx>>,
                $args: Vec<crate::value::Value>,
            ) -> Result<crate::value::Value, crate::interpreter::RuntimeError> {
                $body
            }

            fn arity(&self) -> usize {
                $arity
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({})", $fn_name)
            }
        }
    };
}

pub(crate) use builtin;
