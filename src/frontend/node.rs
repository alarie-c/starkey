#[derive(Debug)]
pub enum Node<'a> {
    // Atoms
    Ident(&'a str),
    Float(f32),
    Itenger(i32),
    Str(&'a str),

    // Other
    QIdent(Box<Node<'a>>, Box<Node<'a>>),

}