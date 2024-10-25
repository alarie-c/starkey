use crate::frontend::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    stream: &'a [u8],
    pos: usize,
    output: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            stream: source.as_bytes(),
            pos: 0usize,
            output: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        loop {
            // Check for EOF condition
            if self.pos >= self.stream.len() {
                self.pos = self.stream.len(); // set pos to length in case pos > len
                self.add_token(TokenKind::EOF, self.pos, 1);
                break;
            }

            // Attempt to match a token
            match self.stream[self.pos..] {
                [b'-', b'>', ..] => self.add_token(TokenKind::Arrow, self.pos, 2),
                [b':', b':', ..] => self.add_token(TokenKind::ColonColon, self.pos, 2),
                [b'=', b'=', ..] => self.add_token(TokenKind::EqualEqual, self.pos, 2),
                [b'!', b'=', ..] => self.add_token(TokenKind::BangEqual, self.pos, 2),
                [b'<', b'=', ..] => self.add_token(TokenKind::LessEqual, self.pos, 2),
                [b'>', b'=', ..] => self.add_token(TokenKind::MoreEqual, self.pos, 2),
                [b'=', ..] => self.add_token(TokenKind::Equal, self.pos, 1),
                [b'-', ..] => self.add_token(TokenKind::Minus, self.pos, 1),
                [b'+', ..] => self.add_token(TokenKind::Plus, self.pos, 1),
                [b'/', ..] => self.add_token(TokenKind::Slash, self.pos, 1),
                [b'*', ..] => self.add_token(TokenKind::Star, self.pos, 1),
                [b'%', ..] => self.add_token(TokenKind::Modulo, self.pos, 1),
                [b'^', ..] => self.add_token(TokenKind::Exponent, self.pos, 1),
                [b':', ..] => self.add_token(TokenKind::Colon, self.pos, 1),
                [b'<', ..] => self.add_token(TokenKind::Less, self.pos, 1),
                [b'>', ..] => self.add_token(TokenKind::More, self.pos, 1),
                [b'!', ..] => self.add_token(TokenKind::Bang, self.pos, 1),
                [b'.', ..] => self.add_token(TokenKind::Dot, self.pos, 1),
                [b'"', ..] => {
                    let begin = self.pos;
                    let literal = self.str();
                    match literal {
                        Some(s) => {
                            let len = s.len() + 2;
                            self.add_token(TokenKind::Str(s), begin, len);
                        }
                        None => todo!("Non-terminating literal"),
                    }
                },
                _ => {
                    // Tokenize number literals
                    if self.stream[self.pos].is_ascii_digit() {
                        let begin = self.pos;
                        let num = self.number();
                        let len = num.len();
                        self.add_token(TokenKind::Number(num), begin, len);

                    // Tokenize identifiers or keywords
                    } else if self.stream[self.pos].is_ascii_alphanumeric() {
                        let begin = self.pos;
                        let ident = self.ident();

                        // Look for keywords
                        match ident {
                            "let" => self.add_token(TokenKind::Let, begin, 3),
                            "const" => self.add_token(TokenKind::Const, begin, 5),
                            "if" => self.add_token(TokenKind::If, begin, 2),
                            "else" => self.add_token(TokenKind::Else, begin, 4),
                            "elif" => self.add_token(TokenKind::Elif, begin, 4),
                            "end" => self.add_token(TokenKind::End, begin, 3),
                            _ => {
                                let len = ident.len();
                                self.add_token(TokenKind::Ident(ident), begin, len);
                            },
                        }
                        
                        self.pos -= 1;
                    }
                },
            }

            // Advance position
            self.pos += 1;
        }

        &self.output
    }

    fn str(&mut self) -> Option<&'a str> {
        let start = self.pos + 1;
        loop {
            self.pos += 1;
            if self.pos >= self.stream.len() {
                todo!("Non-terminating string literal")
            } else if self.stream[self.pos] != b'"' {
                continue;
            } else {
                // Calling unwrap() here because if the source file contains non-utf8 chars it should
                // stop the compiler before the lexer is initialized...
                return Some(std::str::from_utf8(&self.stream[start..self.pos - 1]).unwrap());
            }
        }
    }

    fn number(&mut self) -> &'a str {
        let start = self.pos;
        loop {
            self.pos += 1;
            if self.pos >= self.stream.len() {
                break;
            } else if !self.stream[self.pos].is_ascii_digit()
                && self.stream[self.pos] != b'_'
                && self.stream[self.pos] != b'.'
            {
                self.pos -= 1; // move pos back to tokenize() can deal with the char we just consumed
                break;
            }
        }
        return std::str::from_utf8(&self.stream[start..self.pos]).unwrap();
    }

    fn ident(&mut self) -> &'a str {
        let start = self.pos;
        loop {
            self.pos += 1;
            if self.pos >= self.stream.len() {
                break;
            } else if !self.stream[self.pos].is_ascii_alphanumeric()
                && self.stream[self.pos] != b'_'
            {
                break;
            }
        }
        return std::str::from_utf8(&self.stream[start..self.pos]).unwrap();
    }

    fn add_token(&mut self, kind: TokenKind<'a>, begin: usize, width: usize) {
        self.output.push(Token::new(kind, begin, begin + width));
    }
}
