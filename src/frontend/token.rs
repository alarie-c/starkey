/// Token stores the token variant and position in the source code
#[derive(Debug)]
pub struct Token<'a>(pub TokenKind<'a>, pub TokenSpan);

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, begin: usize, end: usize) -> Self {
        Self(kind, TokenSpan(begin, end - 1))
    }
}

/// Stores beginning and end, inclusive
#[derive(Debug)]
pub struct TokenSpan(pub usize, pub usize);

/// Variants for every kind of token recognized by the program
/// Anything non-enumerated (e.g. string literals, numbers, and symbols) is stored
/// in its variant's field (Str, Number, Ident) respectively
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind<'a> {
    // Grouping
    LPar,
    RPar,
    LBrac,
    RBrac,
    LCurl,
    RCurl,

    // Operators
    Arrow,
    Colon,
    ColonColon,
    SemiColon,
    Dot,
    DotDot,
    Print,
    Comma,
    QMark,

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
    Var,
    Const,
    If,
    Else,
    Elif,
    Def,
    From,
    Import,
    Is,
    Class,
    Enum,
    Return,
    Flag,
    Methods,
    This,

    // Other
    EOF,
}
