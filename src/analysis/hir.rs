use crate::node::{Node, VariableAssignment};

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    Str,
}

#[derive(Debug)]
pub enum Data {
    Int(i32),
    Float(f32),
    Str(String),
    Ident(String),
}

#[derive(Debug)]
pub enum Op {
    Variable { name: String, value: Data, typ: Type },
    Const { name: String, value: Data, typ: Type },
    Exit { code: Data },
}

pub fn generate(mut ast: Vec<Node>) -> Vec<Op> {
    let mut hir = Vec::<Op>::new();

    ast.drain(0..).for_each(|n| {
        match n {
            Node::VariableAssignment(v) => hir.push(variable(&v)),
            Node::Exit(code) => hir.push(Op::Exit{code: Data::Int(code)}),
            _ => {}
        }
    });

    hir
}

fn variable(v: &VariableAssignment) -> Op {
    let typ: Type;
    let value: Data;

    match v.value.as_ref() {
        Node::Integer(val) => {
            typ = Type::Int;
            value = Data::Int(*val);
            if v.typ.is_some() {
                match v.typ.as_ref().unwrap().as_ref() {
                    Node::Ident(s) => {
                        if s.as_str() != "int" {
                            todo!("error int assigned to {}", s);
                        }
                    },
                    _ => todo!("error non-ident type name"),
                }
            }
        },
        Node::Float(val) => {
            typ = Type::Float;
            value = Data::Float(*val);
            if v.typ.is_some() {
                match v.typ.as_ref().unwrap().as_ref() {
                    Node::Ident(s) => {
                        if s.as_str() != "float" {
                            todo!("error float assigned to {}", s);
                        }
                    },
                    _ => todo!("error non-ident type name"),
                }
            }
        },
        Node::Str(val) => {
            typ = Type::Str;
            value = Data::Str(val.to_owned());
            if v.typ.is_some() {
                match v.typ.as_ref().unwrap().as_ref() {
                    Node::Ident(s) => {
                        if s.as_str() != "string" {
                            todo!("error string assigned to {}", s);
                        }
                    },
                    _ => todo!("error non-ident type name"),
                }
            }
        },
        _ => todo!("error non-recognized type"),
    }
    
    Op::Variable {
        name: match v.ident.as_ref() {
            Node::Ident(name) => name.to_owned(),
            _ => todo!("error var assigned non-ident name"),
        },
        value,
        typ,
    }
}