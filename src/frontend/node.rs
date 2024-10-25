#[derive(Debug)]
pub enum Node<'a> {
    AccessMember {
        parent: Box<Self>,
        child: Box<Self>,
    },

    // Values
    Float(f32),
    Integer(i32),
    Str(&'a str),
    Ident(&'a str),

    // Variables & Values
    Var {
        ident: Box<Self>,
        value: Box<Self>,
        typ: Option<Box<Self>>,
    },
    New {
        ident: Box<Self>,
        value: Box<Self>,
        typ: Option<Box<Self>>,
    },
    Set {
        ident: Box<Self>,
        value: Box<Self>,
    },

    // Other
    Exit(i32),
}
