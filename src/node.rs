#[derive(Debug)]
pub enum Node {
    VariableAssignment {
        ident: Box<Node>,
        constant: bool,
        value: Box<Node>,
        typ: Option<Box<Node>>,
    },

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
pub struct BinaryExpression {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: u8,
}
