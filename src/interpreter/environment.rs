use std::collections::HashMap;

use crate::{lox_runtime_error, scanner::token::Token, utils::literal_value::LiteralValue};

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: Token) -> LiteralValue {
        match self.values.get(&name.lexame) {
            Some(value) => return value.clone(),
            None => lox_runtime_error(name.clone(), format!("Undefined variable {}", &name.lexame)),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }
}
