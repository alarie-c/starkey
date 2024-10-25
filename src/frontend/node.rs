#[derive(Debug)]
pub enum Node<'a> {
    VariableAssignment {
        ident: Box<Self>,
        constant: bool,
        value: Box<Self>,
        typ: Option<Box<Self>>,
    },

    AccessMember {
        parent: Box<Self>,
        child: Box<Self>,
    },

    // Values
    Float(f32),
    Integer(i32),
    Str(&'a str),
    Ident(&'a str),

    // Other
    Exit(i32),
}
