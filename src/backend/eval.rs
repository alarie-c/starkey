use std::{cell::RefCell, collections::HashMap, iter::Peekable};

use crate::frontend::expr::Expr;

use super::{context::Context, value::Value};

pub struct Runtime<'r, Iter: Iterator<Item = Expr>> {
    pub globals: HashMap<String, Value<'r>>,
    pub contexts: Vec<Context<'r>>,
    tree: Iter,
}

impl<'r, Iter: Iterator<Item = Expr>> Runtime<'r, Iter> {
    pub fn initialize(tree: Iter) -> Self {
        Self {
            globals: HashMap::new(),
            contexts: Vec::new(),
            tree
        }
    }

    pub fn evaluate(&mut self) {
        while let Some(e) = self.tree.next() {
            self.eval_expr(&e);
        }
    }

    fn eval_expr(&mut self, expr: &Expr) {

    }
}