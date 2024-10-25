/// Token stores the token variant and position in the source code
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan,
}

impl Token {
    pub fn new(kind: TokenKind, begin: usize, end: usize) -> Self {
        Self {
            kind,
            span: TokenSpan(begin, end - 1),
        }
    }
}

/// Stores beginning and end, inclusive
#[derive(Debug)]
pub struct TokenSpan(usize, usize);

/// Variants for every kind of token recognized by the program
/// Anything non-enumerated (e.g. string literals, numbers, and symbols) is stored
/// in its variant's field (Str, Number, Ident) respectively
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Operators
    Arrow,
    Colon,
    ColonColon,

    // Comparison
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,
    
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Exponent,

    // Literals
    Ident(String),
    Number(String),
    Str(String),

    // Keywords
    Let,
    Const,
    If,
    Else,
    Elif,
    End,

    // Other
    EOF,
}

impl TokenKind {
    pub fn is_branch_node(&self) -> bool {
        match self {
            &TokenKind::Ident(_) => false,
            &TokenKind::Str(_) => false,
            &TokenKind::Number(_) => false,
            _ => true,
        }
    }
}
