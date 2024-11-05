use std::iter::Peekable;

use super::{
    expr::Expr,
    token::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Empty,
    UntypedVarExpr,
    UntypedConstExpr,
    TypedVarExpr,
    TypedConstExpr,
    MutationExpr,
}

#[derive(Debug)]
pub struct Parser<'a, Iter: Iterator<Item = &'a Token<'a>>> {
    tokens: Peekable<Iter>,
    stack: Vec<Expr>,
    tree: Vec<Expr>,
    state: State,
}

impl<'a, Iter: Iterator<Item = &'a Token<'a>>> Parser<'a, Iter> {
    pub fn new(tokens: Iter) -> Self {
        Self {
            tokens: tokens.peekable(),
            stack: Vec::new(),
            tree: Vec::new(),
            state: State::Empty,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.tokens.next() {
            self.parse_expr(token);
        }
    }

    fn try_reduce(&mut self) -> Option<()> {
        println!("Attempting to reduce the stack");
        println!("State = {:?}", self.state);
        dbg!(&self.stack);
        dbg!(&self.tree);

        match self.state {
            State::UntypedVarExpr => self.reduce_var_expr(false, false),
            State::UntypedConstExpr => self.reduce_var_expr(false, true),
            State::TypedVarExpr => self.reduce_var_expr(true, false),
            State::TypedConstExpr => self.reduce_var_expr(true, true),
            State::MutationExpr => self.reduce_mutation(),
            _ => panic!("Unexpected state {:?}", self.state),
        }
    }

    fn reduce_mutation(&mut self) -> Option<()> {
        if self.stack.len() == 2 {
            // Ident -> Value;
            let value = self.stack.pop().unwrap();
            let name = self.stack.pop().unwrap();
            self.tree
                .push(Expr::MutateExpr(Box::new(name), Box::new(value)));
            Some(())
        } else {
            None
        }
    }

    fn reduce_var_expr(&mut self, typed: bool, constant: bool) -> Option<()> {
        if self.stack.len() == 2 && !typed {
            // Variable assignment, no type annotation
            let value = self.stack.pop().unwrap();
            let name = self.stack.pop().unwrap();
            if constant {
                self.tree
                    .push(Expr::ConstExpr(Box::new(name), None, Box::new(value)));
            } else {
                self.tree
                    .push(Expr::VariableExpr(Box::new(name), None, Box::new(value)));
            }
            Some(())
        } else if self.stack.len() == 3 && typed {
            // Variable assignment, with type annotation
            let value = self.stack.pop().unwrap();
            let typ = self.stack.pop().unwrap();
            let name = self.stack.pop().unwrap();
            if constant {
                self.tree.push(Expr::ConstExpr(
                    Box::new(name),
                    Some(Box::new(typ)),
                    Box::new(value),
                ));
            } else {
                self.tree.push(Expr::VariableExpr(
                    Box::new(name),
                    Some(Box::new(typ)),
                    Box::new(value),
                ));
            }
            Some(())
        } else {
            None
        }
    }

    fn parse_expr(&mut self, token: &'a Token) {
        match token.0 {
            TokenKind::Number(n) => self.expr_number(n),
            TokenKind::Ident(i) => self.expr_ident(i),
            TokenKind::Dot => self.expr_qualified_ident(),
            TokenKind::EOF => println!("ENDING"),
            TokenKind::Var => self.state = State::UntypedVarExpr,
            TokenKind::Const => self.state = State::UntypedConstExpr,
            TokenKind::Arrow => self.state = State::MutationExpr,

            TokenKind::Equal => match self.state {
                State::UntypedVarExpr
                | State::TypedVarExpr
                | State::TypedConstExpr
                | State::UntypedConstExpr => {}
                _ => panic!("Unexpected `=` in state: {:?}", self.state),
            },

            TokenKind::Colon => {
                if self.state == State::UntypedVarExpr {
                    self.state = State::TypedVarExpr;
                } else if self.state == State::UntypedConstExpr {
                    self.state = State::TypedConstExpr;
                }
            }

            TokenKind::SemiColon => match self.try_reduce() {
                Some(_) => self.state = State::Empty,
                None => {
                    eprintln!("There was an error reducing the stack!");
                    dbg!(&self.stack);
                    dbg!(&self.tree);
                }
            },
            TokenKind::Equal => {}
            _ => panic!("Unexpected token! {:?}", token),
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
            self.stack
                .push(Expr::QualifiedIdent(Box::new(left), Box::new(right)));
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
                Err(_) => panic!("Error parsing integer"),
            };
        } else {
            match number.parse::<i32>() {
                Ok(v) => self.stack.push(Expr::Integer(v)),
                Err(_) => panic!("Error parsing integer"),
            };
        }
    }
}
