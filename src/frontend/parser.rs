use super::{node::Node, token::{Token, TokenKind}};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    stack: Vec<&'a Token<'a>>,
    tree: Vec<Node>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            stack: Vec::new(),
            tree: Vec::new(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) {
        while self.pos < self.tokens.len() {
            // Push the token to the stack
            // self.stack.push(self.tokens.get(self.pos).unwrap());
            
            // Attempt to reduce whatever's on the stack
            // match self.try_reduce() {
            //     Some(n) => self.tree.push(n),
            //     None => {}
            // }

            let token = self.tokens.get(self.pos).unwrap();
            match self.parse_token(token) {
                Some(n) => self.tree.push(n),
                None => {},
            }

            // Advance
            self.pos += 1;
        }
    }

    fn parse_token(&mut self, token: &'a Token) -> Option<Node> {
        match token {
            Token(TokenKind::Str(s), ..) => Some(self.parse_str(s)),
            Token(TokenKind::Number(n), ..) => Some(self.parse_number(n)),
            Token(TokenKind::Ident(i), ..) => Some(self.parse_ident(i)),
            Token(TokenKind::Dot, ..) => Some(self.parse_qualified_ident()),
            _ => None,
        }
    }

    fn parse_qualified_ident(&mut self) -> Node {
        let left = self.tree.pop().unwrap_or_else(|| {
            panic!("No LHS identifer for QualifiedIdent")
        });

        // Get the next token and attempt to get it into a node
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
            let new_token = self.tokens.get(self.pos).unwrap();
            let right = self.parse_token(new_token).unwrap_or_else(|| {
                panic!("Expected a valid node for RHS ")
            });
            return Node::QualifiedIdent(Box::new(left), Box::new(right));
        } else {
            panic!("No RHS identifier for QualifiedIdent")
        }
    }

    fn parse_ident(&mut self, ident: &'a str) -> Node {
        Node::Ident(ident.to_string())
    }

    /// Attempts to reduce by comparing the stack to rules
    /// Will pop the tokens that are reduced and return them
    /// Returns `None` if nothing is able to be reduced.
    // fn try_reduce(&mut self) -> Option<Node> {
    //     match self.stack.as_slice() {
    //         [Token(TokenKind::Str(s), ..), ..] => Some(self.parse_str(s)),
    //         [Token(TokenKind::Number(n), ..), ..] => Some(self.parse_number(n)),
    //         _ => None,
    //     }
    // }

    fn parse_str(&mut self, string: &'a str) -> Node {
        //let _ = self.stack.pop();
        Node::Str(string.to_string())
    }

    fn parse_number(&mut self, number: &'a str) -> Node {
        //let _ = self.stack.pop();
        if number.contains('.') {
            match number.parse::<f32>() {
                Ok(v) => return Node::Float(v),
                Err(_) => panic!("Error parsing integer")
            };
        } else {
            match number.parse::<i32>() {
                Ok(v) => return Node::Integer(v),
                Err(_) => panic!("Error parsing integer")
            };
        }
    }
}