#[derive(Debug)]
pub enum Val {
    Str(String),
    Int(i32),
    Float(f32),
    Identifier(String),
}

#[derive(Debug)]
pub enum Instruction {
    /// Used for variable/const declarations
    /// Stores `Val` with the name `String`
    Store(String, Val),
    StoreConst(String, Val),

    // Basic arithmetic
    Add(Val, Val),
    Sub(Val, Val),
    Mul(Val, Val),
    Div(Val, Val),
    Mod(Val, Val),
    Exp(Val, Val),
}
