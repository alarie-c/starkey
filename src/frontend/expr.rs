#[derive(Debug)]
pub enum Expr {
    // Atoms
    Integer(i32),
    Float(f32),
    Str(String),
    Ident(String),
    Parameter(Box<Expr>, Box<Expr>),
    QMark(Box<Expr>),
    Wildcard,

    QualifiedIdent(Box<Expr>, Box<Expr>),

    ParensExpr(Box<Expr>),
    BlockExpr(Vec<Box<Expr>>),

    BinaryExpr(Box<Expr>, Box<Expr>, BinaryOperator),

    PrintExpr(Box<Expr>),

    /// Ident, Fields
    ClassExpr(Box<Expr>, Box<Expr>),

    /// Expects `Vec<Expr::Parameter>`
    ClassFields(Vec<Box<Expr>>),

    /// Expects Ident and `Expr::BlockExpr`
    ClassMethods(Box<Expr>, Vec<Box<Expr>>),

    /// from `Package` import `Vec<Symbols>`
    ImportExpr(Box<Expr>, Box<Expr>),
    ImportArgs(Vec<Box<Expr>>),
    Directive(Box<Expr>),

    /// Ident, Params, Return, Body
    FunctionExpr(Box<Expr>, Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    /// Ident, Arguments
    FunctionCall(Box<Expr>, Box<Expr>),
    FunctionArgs(Vec<Box<Expr>>),
    ReturnExpr(Box<Expr>),

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
