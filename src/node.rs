#[derive(Debug)]
pub enum Node {
    VariableAssignment(VariableAssignment),

    // Values
    Float(f32),
    Integer(i32),
    Str(String),
    Ident(String),
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub ident: Box<Node>,
    pub constant: bool,
    pub value: Box<Node>,
    pub typ: Option<Box<Node>>,
}
