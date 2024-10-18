// use crate::node::{Node, VariableAssignment};
// use std::{cell::RefCell, collections::HashMap};

// #[derive(Debug)]
// pub enum Value {
//     Integer(i32),
//     Float(f32),
//     Str(String),
//     Bool(bool),
// }

// #[derive(Debug)]
// pub struct Context {
//     pub const_data: HashMap<String, Value>,
//     pub data: HashMap<String, Value>,
// }

// impl Context {
//     pub fn empty() -> Self {
//         Self {
//             const_data: HashMap::new(),
//             data: HashMap::new(),
//         }
//     }

//     pub fn get(&self, k: &String) -> Option<&Value> {
//         self.data.get(k)
//     }

//     pub fn set(&mut self, k: &String, v: Value) {
//         self.data.insert(k.to_owned(), v);
//     }

//     pub fn set_const(&mut self, k: &String, v: Value) {
//         self.const_data.insert(k.to_owned(), v);
//     }

//     pub fn get_const(&mut self, k: &String, v: Value) -> Option<&Value> {
//         self.data.get(k)
//     }
// }

// #[derive(Debug)]
// pub struct Evaluator {
//     //tree: RefCell<Vec<Node>>,
//     context: Context,
// }

// impl Evaluator {
//     pub fn new() -> Self {
//         Self {
//             //tree: RefCell::new(tree),
//             context: Context::empty(),
//         }
//     }

//     pub fn eval(&mut self, tree: Vec<Node>) {
//         for n in tree.iter() {
//             self.eval_expr(&n);
//         }
//     }

//     fn eval_expr(&mut self, node: &Node) {
//         match node {
//             Node::VariableAssignment(VariableAssignment {
//                 ident: i,
//                 constant: c,
//                 value: v,
//                 typ: t,
//             }) => self.variable_assignment(i, c, v, t),
//             _ => {}
//         }
//     }

//     fn variable_assignment(
//         &mut self,
//         i: &Box<Node>,
//         c: &bool,
//         v: &Box<Node>,
//         _t: &Option<Box<Node>>,
//     ) {
//         match v.as_ref() {
//             Node::Str(value) => match i.as_ref() {
//                 Node::Ident(name) => {
//                     if *c {
//                         self.context.set_const(name, Value::Str(value.to_owned()));
//                     } else {
//                         self.context.set(name, Value::Str(value.to_owned()));
//                     }
//                 }
//                 _ => todo!("error invalid ident for var"),
//             },
//             _ => {}
//         }
//     }
// }
