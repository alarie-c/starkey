use super::context::Context;

pub enum Type {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    Nil,
}

pub struct Value<'r> {
    pub typ: Type,
    pub name: String,
    pub constant: bool,
    pub context: &'r Context<'r>,
}