use std::iter::Peekable;

use super::{
    expr::{BinaryOperator, Expr},
    token::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Empty,
    PreParamFunctionExpr,
    PostParamFunctionExpr,
    PrintExpr,
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
            State::PrintExpr => self.reduce_print_expr(),
            State::UntypedVarExpr => self.reduce_var_expr(false, false),
            State::UntypedConstExpr => self.reduce_var_expr(false, true),
            State::TypedVarExpr => self.reduce_var_expr(true, false),
            State::TypedConstExpr => self.reduce_var_expr(true, true),
            State::MutationExpr => self.reduce_mutation(),
            State::PostParamFunctionExpr => self.reduce_function_expr(),
            _ => panic!("Unexpected state {:?}", self.state),
        }
    }

    fn reduce_function_expr(&mut self) -> Option<()> {
        // No return annotation
        if self.stack.len() == 3 {
            let block = self.stack.pop().unwrap();
            let params = self.stack.pop().unwrap();
            let ident = self.stack.pop().unwrap();
            self.tree.push(Expr::FunctionExpr(Box::new(ident), Box::new(params), None, Box::new(block)));
            Some(())
        // Valid return annotation
        } else if self.stack.len() == 4 {
            let block = self.stack.pop().unwrap();
            let returns = self.stack.pop().unwrap();
            let params = self.stack.pop().unwrap();
            let ident = self.stack.pop().unwrap();
            self.tree.push(Expr::FunctionExpr(Box::new(ident), Box::new(params), Some(Box::new(returns)), Box::new(block)));
            Some(())
        } else {
            None
        }
    }

    fn reduce_print_expr(&mut self) -> Option<()> {
        if self.stack.len() >= 1 {
            let expr = self.stack.pop().unwrap();
            self.tree
                .push(Expr::PrintExpr(Box::new(expr)));
            Some(())
        } else {
            None
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
        // Variable assignment, no type annotation
        if self.stack.len() == 2 && !typed {
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

        // Variable assignment, with type annotation
        } else if self.stack.len() == 3 && typed {
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

        // Something is wrong with the organization of the stack
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
            TokenKind::Def => self.state = State::PreParamFunctionExpr,

            TokenKind::Arrow => self.state = State::MutationExpr,

            TokenKind::LPar => {
                match self.state {
                    State::PreParamFunctionExpr => self.expr_parameters(),
                    _ => self.expr_parens(),
                }
            }

            TokenKind::LCurl => {
                match self.state {
                    State::PostParamFunctionExpr => self.expr_block(),
                    _ => panic!("Unexpected `{{` in state: {:?}", self.state),
                }
            }

            TokenKind::Print => self.state = State::PrintExpr,

            TokenKind::Plus => self.expr_binaryop(BinaryOperator::Plus),
            TokenKind::Minus => self.expr_binaryop(BinaryOperator::Minus),
            TokenKind::Star => self.expr_binaryop(BinaryOperator::Multiply),
            TokenKind::Slash => self.expr_binaryop(BinaryOperator::Divide),
            TokenKind::Modulo => self.expr_binaryop(BinaryOperator::Modulo),
            TokenKind::Exponent => self.expr_binaryop(BinaryOperator::Exponent),

            TokenKind::Equal => match self.state {
                // Make sure equal is being used in the correct state here
                State::UntypedVarExpr
                | State::TypedVarExpr
                | State::TypedConstExpr
                | State::UntypedConstExpr => {}
                _ => panic!("Unexpected `=` in state: {:?}", self.state),
            },

            TokenKind::Colon => {
                // Again, make sure colon is being used in the correct state here
                match self.state {
                    State::UntypedVarExpr => self.state = State::TypedVarExpr,
                    State::UntypedConstExpr => self.state = State::TypedConstExpr,
                    State::PreParamFunctionExpr => self.expr_parameter(),
                    State::PostParamFunctionExpr => {},
                    _ => panic!("Unexpected `:` in state: {:?}", self.state),
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
            _ => panic!("Unexpected token! {:?}", token),
        }
    }

    fn expr_parameter(&mut self) {
        let ident = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid LHS identifier for parameter");
        });

        if let Some(token) = self.tokens.next() {
            self.parse_expr(token);
            // Make sure we keep parsing just in case the identifier is qualified
            while let Some(token) = self.tokens.peek() {
                if token.0 != TokenKind::Dot { break; }
                let token = self.tokens.next().unwrap();
                self.parse_expr(token);
            }
            let typ = self.stack.pop().unwrap_or_else(|| {
                panic!("Expected a valid RHS identifier for parameter");
            });
            self.stack
                .push(Expr::Parameter(Box::new(ident), Box::new(typ)));
        } else {
            panic!("No RHS identifier for parameter")
        }
    }
    
    fn expr_parameters(&mut self) {
        let start_len = self.stack.len();
        while let Some(token) = self.tokens.next() {
            if token.0 == TokenKind::RPar { break; }
            self.parse_expr(token);
        } 
        self.state = State::PostParamFunctionExpr;
        // Take everything EXCEPT the function's identifier off the stack
        let mut params = Vec::<Box<Expr>>::new();
        while self.stack.len() != start_len {
            // This code will only take off as many as we put on, so .unwrap() is appropriate
            let expr = self.stack.pop().unwrap();
            params.push(Box::new(expr));
        }
        self.stack.push(Expr::ParametersExpr(params));
    }

    fn expr_parens(&mut self) {
        while let Some(token) = self.tokens.next() {
            if token.0 == TokenKind::RPar { break; }
            self.parse_expr(token);
        } 
        let expr = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid expression for ParensExpr");
        });
        // This will only push the last thing on the stack!!! If something went wrong and it wasn't
        // reduced to just 1 Expr then there will be an undetected parse error...
        // TODO: Fix that ^^^^^^^^
        self.stack.push(Expr::ParensExpr(Box::new(expr)));
    }

    fn expr_block(&mut self) {
        // Effectively hi-jacking the rest of the parser, taking things off the tree and sticking them
        // into the block expression
        let mut len = self.tree.len();
        let mut block = Vec::<Box<Expr>>::new();
        let original_stack: Vec<Expr> = self.stack.drain(0..).collect();
        let original_state = self.state.clone();
        while let Some(token) = self.tokens.next() {
            if token.0 == TokenKind::RCurl { break; }
            self.parse_expr(token);
            // When something was added to the tree
            if self.tree.len() > len {
                // Yank and shove in the BlockExpr
                let expr = self.tree.pop().unwrap();
                block.push(Box::new(expr));
                len = self.tree.len();
            }
        }
        // Push the original stack
        self.stack = original_stack;
        self.stack.push(Expr::BlockExpr(block));
        self.state = original_state;
        if self.state == State::PostParamFunctionExpr {
            match self.try_reduce() {
                Some(_) => self.state = State::Empty,
                None => {
                    eprintln!("There was an error reducing the stack!");
                    dbg!(&self.stack);
                    dbg!(&self.tree);
                }
            }
        }
    }

    fn expr_binaryop(&mut self, operator: BinaryOperator) {
        let left = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid LHS expression for BinOp");
        });

        if let Some(token) = self.tokens.next() {
            self.parse_expr(token);
            let right = self.stack.pop().unwrap_or_else(|| {
                panic!("Expected a valid RHS expression for BinOp");
            });
            self.stack
                .push(Expr::BinaryExpr(Box::new(left), Box::new(right), operator));
        } else {
            panic!("No RHS expression for BinOp")
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