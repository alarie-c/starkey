use std::collections::HashMap;

use super::value::Value;

pub enum Op<T> {
    New { name: String, value: Value<T> },
    Var { name: String, value: Value<T> },
    Set { name: String, value: Value<T> },
    Log { expr: Value<T> },
}

impl<T> Op<T> {
    pub fn exec(&self, context: &mut HashMap<String, Value<T>>) {
    } 
}

fn op_new<T>(context: &mut HashMap<String, Value<T>>, name: String, value: Value<T>) {
    context.insert(name, value);
}