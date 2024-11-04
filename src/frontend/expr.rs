#[derive(Debug)]
pub enum Expr {
    // Atoms
    Integer(i32),
    Float(f32),
    Str(String),
    Ident(String),

    QualifiedIdent(Box<Expr>, Box<Expr>),
    
    BinaryExpr(Box<Expr>, Box<Expr>, char),
    
    VariableExpr(Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    ConstExpr(Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    MutateExpr(Box<Expr>, Box<Expr>),
}