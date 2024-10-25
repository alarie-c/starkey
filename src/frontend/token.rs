/// Token stores the token variant and position in the source code
#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: TokenSpan,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, begin: usize, end: usize) -> Self {
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
pub enum TokenKind<'a> {
    // Operators
    Arrow,
    Colon,
    ColonColon,
    Dot,

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
    Ident(&'a str),
    Number(&'a str),
    Str(&'a str),

    // Keywords
    Let,
    Const,
    If,
    Else,
    Elif,
    End,

    // Other
    Newline,
    EOF,
}

impl<'a> TokenKind<'a> {
    pub fn is_branch_node(&self) -> bool {
        match self {
            &TokenKind::Ident(_) => false,
            &TokenKind::Str(_) => false,
            &TokenKind::Number(_) => false,
            _ => true,
        }
    }
}
