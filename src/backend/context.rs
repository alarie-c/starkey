use std::collections::HashMap;

use super::value::Value;

/// Contexts are the owners and handlers of all their values
/// Values store a reference to thier context and their address
/// in that context, but otherwise they exist entirely independent
/// of scope and context
pub struct Context<'r> {
    pub locals: HashMap<String, Value<'r>>,
}

impl<'r> Context<'r> {
    pub fn new() -> Self {
        Self { locals: HashMap::new() }
    }

    pub fn store(&mut self, key: String, value: Value<'r>) {
        self.locals.insert(key, value).unwrap();
    }  

    pub fn get(&mut self, key: &String) -> Option<&Value<'r>> {
        self.locals.get(key)
    }

    /// Attempts to set the key to the value provided
    /// If a value doesn't exist at that key, it will return `None`
    /// If a value does exist but is constant, then it will return `None`
    /// Will return `Some(())` when there is a value at the key and the value is not constant
    pub fn set(&mut self, key: &String, value: Value<'r>) -> Option<()> {
        if let Some(v) = self.locals.get(key) {
            if v.constant { return None }
            self.locals.insert(key.clone(), value).unwrap();
            return Some(())
        } else {
            return None
        }
    }
}