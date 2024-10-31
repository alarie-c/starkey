#[derive(Debug)]
pub enum Node {
    // Atoms
    Integer(i32),
    Float(f32),
    Str(String),
    Ident(String),

    QualifiedIdent(Box<Node>, Box<Node>),
    BinaryExpr(Box<Node>, Box<Node>, char),
}