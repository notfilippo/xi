use std::fmt::Display;

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct List {
    pub items: Vec<Value>,
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for e in self.items.iter().take(1) {
            write!(f, "{}", e)?;
        }
        for e in self.items.iter().skip(1) {
            write!(f, ", {}", e)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}
