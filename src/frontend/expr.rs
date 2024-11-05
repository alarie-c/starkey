#[derive(Debug)]
pub enum Expr {
    // Atoms
    Integer(i32),
    Float(f32),
    Str(String),
    Ident(String),
    Parameter(Box<Expr>, Box<Expr>),

    QualifiedIdent(Box<Expr>, Box<Expr>),
    
    ParensExpr(Box<Expr>),
    BlockExpr(Vec<Box<Expr>>),

    BinaryExpr(Box<Expr>, Box<Expr>, BinaryOperator),

    PrintExpr(Box<Expr>),

    /// Ident, Params, Return, Body
    FunctionExpr(Box<Expr>, Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    ParametersExpr(Vec<Box<Expr>>),
    
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