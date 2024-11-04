use std::iter::Peekable;

use super::{expr::Expr, token::{TokenKind, Token}};

#[derive(Debug)]
pub struct Parser<'a, Iter: Iterator<Item = &'a Token<'a>>> {
    tokens: Peekable<Iter>,
    stack: Vec<Expr>,
    tree: Vec<Expr>,
    state: &'a str,
}

impl<'a, Iter: Iterator<Item = &'a Token<'a>>> Parser<'a, Iter> {
    pub fn new(tokens: Iter) -> Self {
        Self {
            tokens: tokens.peekable(),
            stack: Vec::new(),
            tree: Vec::new(),
            state: "",
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.tokens.next() {
            self.parse_expr(token);
        }
    }

    fn try_reduce(&mut self) -> Option<()> {
        println!("Attempting to reduce the stack");
        println!("State = {}", self.state);
        dbg!(&self.stack);
        dbg!(&self.tree);
        
        match self.state {
            "VAR EXPR" => self.reduce_var_expr(false),
            "CONST EXPR" => self.reduce_var_expr(true),
            _ => panic!("Unexpected state {}", self.state),
        }
    }

    fn reduce_var_expr(&mut self, constant: bool) -> Option<()> {
        if self.stack.len() == 2 {
            // Variable assignment, no type annotation
            let value = self.stack.pop().unwrap();
            let name = self.stack.pop().unwrap();
            if constant {
                self.tree.push(Expr::ConstExpr(Box::new(name), None, Box::new(value)));
            } else {
                self.tree.push(Expr::VariableExpr(Box::new(name), None, Box::new(value)));
            }
            Some(())
        } else if self.stack.len() == 3 {
            // Variable assignment, with type annotation
            let value = self.stack.pop().unwrap();
            let name = self.stack.pop().unwrap();
            let typ = self.stack.pop().unwrap();
            if constant {
                self.tree.push(Expr::ConstExpr(Box::new(name), Some(Box::new(typ)), Box::new(value)));
            } else {
                self.tree.push(Expr::VariableExpr(Box::new(name), Some(Box::new(typ)), Box::new(value)));
            }
            Some(())
        } else {
            None
        }
    }

    fn parse_expr(&mut self, token: &'a Token) {
        match token {
            Token(TokenKind::Number(n), ..) => self.expr_number(n),
            Token(TokenKind::Ident(i), ..) => self.expr_ident(i),
            Token(TokenKind::Dot, ..) => self.expr_qualified_ident(),
            Token(TokenKind::EOF, ..) => println!("ENDING"),
            Token(TokenKind::Var, ..) => self.state = "VAR EXPR",
            Token(TokenKind::Const, ..) => self.state = "CONST EXPR",
            Token(TokenKind::SemiColon, ..) => {
                match self.try_reduce() {
                    Some(_) => self.state = "",
                    None => {
                        eprintln!("There was an error reducing the stack!");
                        dbg!(&self.stack);
                        dbg!(&self.tree);
                    }
                }
            }
            Token(TokenKind::Equal, ..) => {}
            _ => panic!("Unexpected token!"),
        }
    }

    fn expr_qualified_ident(&mut self) {
        let left = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid LHS identifier for QI");
        });

        if let Some(token) = self.tokens.next() {
            self.parse_expr(token);
            let right = self.stack.pop().unwrap_or_else(|| {
                panic!("Expected a valid RHS identifier for QI");
            });
            self.stack.push(Expr::QualifiedIdent(Box::new(left), Box::new(right)));
        } else {
            panic!("No RHS identifier for QualifiedIdent")
        }
    }

    fn expr_ident(&mut self, name: &'a str) {
        self.stack.push(Expr::Ident(name.to_string()));
    }

    fn expr_number(&mut self, number: &'a str) {
        if number.contains('.') {
            match number.parse::<f32>() {
                Ok(v) => self.stack.push(Expr::Float(v)),
                Err(_) => panic!("Error parsing integer")
            };
        } else {
            match number.parse::<i32>() {
                Ok(v) => self.stack.push(Expr::Integer(v)),
                Err(_) => panic!("Error parsing integer")
            };
        }
    }
}