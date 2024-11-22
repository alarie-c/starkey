use std::collections::HashMap;

use crate::frontend::expr::Expr;

use super::{context::Context, value::Value};

pub struct Runtime<'r> {
    pub globals: HashMap<String, Value<'r>>,
    pub contexts: Vec<Context<'r>>,
    tree: Vec<Expr>,
}

impl<'r> Runtime<'r> {
    pub fn initialize(tree: Vec<Expr>) -> Self {
        Self {
            globals: HashMap::new(),
            contexts: Vec::new(),
            tree
        }
    }

    pub fn evaluate(&mut self) {
        
    }
}