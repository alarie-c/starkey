#[derive(Debug)]
pub enum Expr {
    // Atoms
    Integer(i32),
    Float(f32),
    Str(String),
    Ident(String),

    QualifiedIdent(Box<Expr>, Box<Expr>),
    
    ParensExpr(Box<Expr>),

    BinaryExpr(Box<Expr>, Box<Expr>, BinaryOperator),

    PrintExpr(Box<Expr>),
    
    VariableExpr(Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    ConstExpr(Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    MutateExpr(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
}