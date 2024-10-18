#[derive(Debug)]
pub enum Node {
    VariableAssignment(VariableAssignment),
    BinaryExpression(BinaryExpression),

    // Values
    Float(f32),
    Integer(i32),
    Str(String),
    Ident(String),

    // Other
    Exit(i32),
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub ident: Box<Node>,
    pub constant: bool,
    pub value: Box<Node>,
    pub typ: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: u8,
}
