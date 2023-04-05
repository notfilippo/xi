use std::{fmt::Display, collections::HashMap};

use crate::value::{Value, ValueKey};

#[derive(Debug, Clone)]
pub struct Dict {
    pub items: HashMap<ValueKey, Value>,
}

impl Display for Dict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (k, v) in self.items.iter().take(1) {
            write!(f, "{}: {}", k, v)?;
        }
        for (k, v) in self.items.iter().skip(1) {
            write!(f, ", {}: {}", k, v)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}
