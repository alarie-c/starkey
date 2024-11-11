use std::slice::Iter;

use crate::frontend::expr::Expr;

use super::{
    context::{Context, GlobalContext},
    instruction::{Instruction, Val},
};

#[derive(Debug)]
pub struct Interep<'ir> {
    instructions: Vec<Instruction>,
    global_ctx: GlobalContext<'ir>,
    parse_tree: Iter<'ir, Expr>,
}

impl<'ir> Interep<'ir> {
    pub fn new(parse_tree: &'ir Vec<Expr>) -> Self {
        Self {
            instructions: Vec::new(),
            global_ctx: GlobalContext(Context::new()),
            parse_tree: parse_tree.iter(),
        }
    }

    pub fn generate(&mut self) {
        while let Some(expr) = self.parse_tree.next() {
            self.build_instruction(expr);
        }
    }

    fn build_instruction(&mut self, expr: &Expr) {
        match expr {
            Expr::VariableExpr(key, typ, value) => self.variable_expr(key, typ, value),
            _ => panic!("Unexpected expression"),
        }
    }

    fn variable_expr(&mut self, key: &Box<Expr>, typ: &Option<Box<Expr>>, value: &Box<Expr>) {
        // Unwrap the key
        let k = match &**key {
            Expr::Ident(s) => s.to_string(),
            _ => panic!("Expected an identifier in variable expr"),
        };

        // Unwrap the type annotation
        // let k = match &**key {
        //     Expr::Ident(s) => s.to_string(),
        //     _ => panic!("Expected an identifier in variable expr"),
        // };

        // Unwrap the value
        let v = match &**value {
            Expr::Float(f) => Val::Float(*f),
            Expr::Integer(i) => Val::Int(*i),
            _ => panic!("Unsupported operation in value of variable")
        };
        
        let inst = Instruction::Store(k, v);
        self.instructions.push(inst);
    }
}
