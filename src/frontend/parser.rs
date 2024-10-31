use std::iter::Peekable;

use super::{node::Node, token::{Token, TokenKind}};

#[derive(Debug)]
pub struct Parser<'a, Iter: Iterator<Item = &'a Token<'a>>> {
    tokens: Peekable<Iter>,
    stack: Vec<&'a Token<'a>>,
    tree: Vec<Node>,
    len: usize
}

impl<'a, Iter: Iterator<Item = &'a Token<'a>>> Parser<'a, Iter> {
    pub fn new(tokens: Iter, len: usize) -> Self {
        Self {
            tokens: tokens.peekable(),
            stack: Vec::new(),
            tree: Vec::new(),
            len,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.tokens.next() {
            match self.parse_token(token) {
                Some(n) => self.tree.push(n),
                None => {},
            }
        }
    }

    fn parse_token(&mut self, token: &'a Token) -> Option<Node> {
        match token {
            Token(TokenKind::Str(s), ..) => Some(self.parse_str(s)),
            Token(TokenKind::Number(n), ..) => Some(self.parse_number(n)),
            Token(TokenKind::Ident(i), ..) => Some(self.parse_ident(i)),
            Token(TokenKind::Dot, ..) => Some(self.parse_qualified_ident()),

            // Arithmetic
            Token(TokenKind::Plus, ..) => Some(self.parse_binary_expr('+')),
            Token(TokenKind::Minus, ..) => Some(self.parse_binary_expr('-')),
            Token(TokenKind::Star, ..) => Some(self.parse_binary_expr('*')),
            Token(TokenKind::Slash, ..) => Some(self.parse_binary_expr('/')),

            // Variables & Constants
            Token(TokenKind::Var, ..) => Some(self.parse_variable_expr(false)),
            Token(TokenKind::Const, ..) => Some(self.parse_variable_expr(true)),
            _ => None,
        }
    }

    fn parse_variable_expr(&mut self, constant: bool) -> Node {
        let ident = Box::new(self.parse_next().unwrap_or_else(|| {
            panic!("Expected identifier for var expr")
        }));

        let mut typ: Option<Box<Node>> = None;
        match self.tokens.peek() {
            // Do type annotation
            Some(peeked_token) => if peeked_token.0 == TokenKind::Colon {
                // Skip the current token (which is a colon)
                let _ = self.tokens.next();
                
                // Parse the new thing (should be an identifier)
                typ = Some(Box::new(self.parse_next().unwrap_or_else(|| {
                    panic!("Expected a type annotation after :")
                })));
            },
            
            // No type annotation
            None => typ = None,
        }

        // Skipp the current token
        let _ = self.tokens.next();
        
        // Look for value of var expr
        let value = Box::new(self.parse_next().unwrap_or_else(|| {
            panic!("Expected a valid value for var expr")
        }));

        // Return the node
        if constant {
            Node::ConstExpr(ident, typ, value)
        } else {
            Node::VariableExpr(ident, typ, value)
        }
    }

    fn parse_qualified_ident(&mut self) -> Node {
        let left = self.tree.pop().unwrap_or_else(|| {
            panic!("No LHS identifer for QualifiedIdent")
        });

        // Get the next token and attempt to get it into a node
        if let Some(right) = self.parse_next() {
            Node::QualifiedIdent(Box::new(left), Box::new(right))
        } else {
            panic!("No RHS identifier for QualifiedIdent")
        }
    }

    fn parse_ident(&mut self, ident: &'a str) -> Node {
        Node::Ident(ident.to_string())
    }

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

    fn parse_binary_expr(&mut self, op: char) -> Node {
        let left = self.tree.pop().unwrap_or_else(|| {
            panic!("No LHS number for BinaryExpr")
        });

        // Get the next token and attempt to get it into a node
        if let Some(right) = self.parse_next() {
            return Node::BinaryExpr(Box::new(left), Box::new(right), op)
        } else {
            panic!("No RHS number for BinaryExpr")
        }
    }

    fn parse_next(&mut self) -> Option<Node> {
        if let Some(token) = self.tokens.next() {
            self.parse_token(token)
        } else {
            None
        }
    }
}