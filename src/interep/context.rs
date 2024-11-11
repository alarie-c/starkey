use std::collections::HashMap;

use super::instruction::Val;

pub enum ContextResult<'ir> {
    Stored,
    Found(&'ir Store<'ir>),
    Mutated,
    ValueAlreadyPresent,
    ValueNotMutable,
    ValueDoesNotExist,
}

#[derive(Debug)]
/// Represents a location in memory and it's value
/// Is stored in a hashmap of it's parent context
/// Tracks the mutability of the value
///
/// Is indexed with a reference to the string under which the
/// value was originally stored
pub struct Store<'ir> {
    context: &'ir Context<'ir>,
    value: Val,
    mutability: bool,
}

#[derive(Debug)]
pub struct Context<'ir> {
    vals: HashMap<&'ir String, Store<'ir>>,
    sub_ctx: Vec<Context<'ir>>,
}

impl<'ir> Context<'ir> {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
            sub_ctx: Vec::new(),
        }
    }

    pub fn store_value(&mut self, key: &'ir String, value: Store<'ir>) -> ContextResult {
        if let Some(_) = self.vals.get(key) {
            return ContextResult::ValueAlreadyPresent;
        }
        self.vals.insert(key, value).unwrap();
        return ContextResult::Stored;
    }

    /// Will attempt to overwrite a value at `key` in the HashMap
    /// If that value isn't mutable, it will return None
    /// If the value doesn't exit, it will return None
    pub fn set_value(&mut self, key: &'ir String, value: Store<'ir>) -> ContextResult {
        match self.vals.get(key) {
            Some(s) => {
                if s.mutability {
                    self.vals.insert(key, value).unwrap();
                    return ContextResult::Mutated;
                } else {
                    return ContextResult::ValueNotMutable;
                }
            }
            None => return ContextResult::ValueDoesNotExist,
        }
    }

    pub fn get_value(&'ir self, key: &'ir String) -> ContextResult<'ir> {
        match self.vals.get(key) {
            Some(s) => return ContextResult::Found(s),
            None => return ContextResult::ValueDoesNotExist,
        }
    }
}

#[derive(Debug)]
pub struct GlobalContext<'ir>(pub Context<'ir>);
