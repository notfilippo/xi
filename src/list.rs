use std::fmt::Display;

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct List(pub Vec<Value>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for e in self.0.iter().take(1) {
            write!(f, "{}", e)?;
        }
        for e in self.0.iter().skip(1) {
            write!(f, ", {}", e)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}
