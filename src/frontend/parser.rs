use std::iter::Peekable;

use super::{
    expr::{BinaryOperator, Expr},
    token::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Empty,
    ReturnExpr,
    ClassExpr,
    ClassMethods,
    PreParamFunctionExpr,
    PostParamFunctionExpr,
    PrintExpr,
    UntypedVarExpr,
    UntypedConstExpr,
    TypedVarExpr,
    TypedConstExpr,
    MutationExpr,
    ImportExpr,
    FlagExpr,
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

    fn reduce(&mut self) -> Option<()> {
        println!("Attempting to reduce the stack");
        println!("State = {:?}", self.state);
        dbg!(&self.stack);
        dbg!(&self.tree);

        match self.state {
            State::ClassExpr => self.reduce_class_expr(),
            State::FlagExpr => self.reduce_flag_expr(),
            State::ReturnExpr => self.reduce_return_expr(),
            State::PrintExpr => self.reduce_print_expr(),
            State::ImportExpr => self.reduce_import_expr(),
            State::UntypedVarExpr => self.reduce_var_expr(false, false),
            State::UntypedConstExpr => self.reduce_var_expr(false, true),
            State::TypedVarExpr => self.reduce_var_expr(true, false),
            State::TypedConstExpr => self.reduce_var_expr(true, true),
            State::MutationExpr => self.reduce_mutation(),
            State::PostParamFunctionExpr => self.reduce_function_expr(),
            State::Empty => match self.stack.last() {
                Some(Expr::FunctionCall(_, _)) => {
                    let fn_call = self.stack.pop().unwrap();
                    self.tree.push(fn_call);
                    Some(())
                }
                _ => panic!("Unexpected state {:?}", self.state),
            },
            _ => panic!("Unexpected state {:?}", self.state),
        }
    }

    fn reduce_class_expr(&mut self) -> Option<()> {
        if self.stack.len() == 2 {
            let fields = self.stack.pop().unwrap();
            let ident = self.stack.pop().unwrap();
            self.tree
                .push(Expr::ClassExpr(Box::new(ident), Box::new(fields)));
            Some(())
        } else {
            None
        }
    }

    fn reduce_flag_expr(&mut self) -> Option<()> {
        if self.stack.len() == 1 {
            let flag = self.stack.pop().unwrap();
            self.tree.push(Expr::FlagExpr(Box::new(flag)));
            Some(())
        } else {
            None
        }
    }

    fn reduce_import_expr(&mut self) -> Option<()> {
        if self.stack.len() == 2 {
            let args = self.stack.pop().unwrap();
            let package = self.stack.pop().unwrap();
            self.tree
                .push(Expr::ImportExpr(Box::new(package), Box::new(args)));
            Some(())
        } else {
            None
        }
    }

    fn reduce_function_expr(&mut self) -> Option<()> {
        // No return annotation
        if self.stack.len() == 3 {
            let block = self.stack.pop().unwrap();
            let params = self.stack.pop().unwrap();
            let ident = self.stack.pop().unwrap();
            self.tree.push(Expr::FunctionExpr(
                Box::new(ident),
                Box::new(params),
                None,
                Box::new(block),
            ));
            Some(())
        // Valid return annotation
        } else if self.stack.len() == 4 {
            let block = self.stack.pop().unwrap();
            let returns = self.stack.pop().unwrap();
            let params = self.stack.pop().unwrap();
            let ident = self.stack.pop().unwrap();
            self.tree.push(Expr::FunctionExpr(
                Box::new(ident),
                Box::new(params),
                Some(Box::new(returns)),
                Box::new(block),
            ));
            Some(())
        } else {
            None
        }
    }

    fn reduce_print_expr(&mut self) -> Option<()> {
        if self.stack.len() >= 1 {
            let expr = self.stack.pop().unwrap();
            self.tree.push(Expr::PrintExpr(Box::new(expr)));
            Some(())
        } else {
            None
        }
    }

    fn reduce_return_expr(&mut self) -> Option<()> {
        if self.stack.len() >= 1 {
            let expr = self.stack.pop().unwrap();
            self.tree.push(Expr::ReturnExpr(Box::new(expr)));
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

    fn try_reduce(&mut self) {
        match self.reduce() {
            Some(_) => self.state = State::Empty,
            None => {
                eprintln!("There was an error reducing the stack!");
                dbg!(&self.stack);
                dbg!(&self.tree);
            }
        };
    }

    fn parse_expr(&mut self, token: &'a Token) {
        match token.0 {
            TokenKind::Number(n) => self.expr_number(n),
            TokenKind::Ident(i) => self.expr_ident(i),
            TokenKind::Str(s) => self.expr_str(s),
            TokenKind::Dot => self.expr_qualified_ident(),
            TokenKind::EOF => println!("ENDING"),

            TokenKind::Var => self.state = State::UntypedVarExpr,
            TokenKind::Const => self.state = State::UntypedConstExpr,
            TokenKind::Def => self.state = State::PreParamFunctionExpr,
            TokenKind::From => self.state = State::ImportExpr,
            TokenKind::Methods => self.state = State::ClassMethods,
            TokenKind::Class => self.state = State::ClassExpr,
            TokenKind::Import => {
                if self.state != State::ImportExpr {
                    panic!("Unexpected `import` in state: {:?}", self.state)
                } else {
                    self.expr_import_args();
                }
            }

            TokenKind::Arrow => self.state = State::MutationExpr,

            TokenKind::LPar => match self.state {
                State::PreParamFunctionExpr => self.expr_parameters(),
                _ => match self.stack.last() {
                    Some(Expr::Ident(_)) => {
                        self.expr_arguments();
                        self.expr_function_call();
                    }
                    _ => self.expr_parens(),
                },
            },

            TokenKind::LCurl => match self.state {
                State::PostParamFunctionExpr => self.expr_block(),
                State::ClassMethods => self.expr_class_methods(),
                State::ClassExpr => self.expr_class_expr(),
                _ => panic!("Unexpected `{{` in state: {:?}", self.state),
            },

            TokenKind::Print => self.state = State::PrintExpr,
            TokenKind::Return => self.state = State::ReturnExpr,
            TokenKind::Flag => self.state = State::FlagExpr,
            TokenKind::QMark => self.expr_qmark(),

            TokenKind::Plus => self.expr_binaryop(BinaryOperator::Plus),
            TokenKind::Minus => self.expr_binaryop(BinaryOperator::Minus),
            TokenKind::Star => match self.state {
                State::ImportExpr => self.stack.push(Expr::Wildcard),
                _ => self.expr_binaryop(BinaryOperator::Multiply),
            },
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
                    State::PreParamFunctionExpr | State::ClassExpr => self.expr_parameter(),
                    State::PostParamFunctionExpr => {}
                    _ => panic!("Unexpected `:` in state: {:?}", self.state),
                }
            }

            TokenKind::SemiColon => self.try_reduce(),
            _ => panic!("Unexpected token! {:?}", token),
        }
    }

    fn expr_class_methods(&mut self) {
        let mut len = self.tree.len();
        let mut methods = Vec::<Box<Expr>>::new();
        let ident = self
            .stack
            .pop()
            .unwrap_or_else(|| panic!("Missing identifier for def block!"));
        while let Some(token) = self.tokens.next() {
            match token.0 {
                TokenKind::Def => self.state = State::PreParamFunctionExpr,
                TokenKind::Ident(i) => self.expr_ident(i),
                TokenKind::Colon => match self.state {
                    State::PreParamFunctionExpr => self.expr_parameter(),
                    State::PostParamFunctionExpr => {}
                    _ => panic!("Unexpected `:` in state: {:?}", self.state),
                },
                TokenKind::LPar => match self.state {
                    State::PreParamFunctionExpr => self.expr_parameters(),
                    _ => panic!("Unexpected '(' in class def block"),
                },
                TokenKind::LCurl => match self.state {
                    State::PostParamFunctionExpr => self.expr_block(),
                    _ => panic!("Unexpected '{{' in class def block"),
                },
                TokenKind::RCurl => break,
                _ => panic!("Unexpected thing in class method block: '{:?}'", token.0),
            }

            if self.tree.len() > len {
                // Snatch that function expr off the tree
                let func = self.tree.pop().unwrap();
                match func {
                    Expr::FunctionExpr(_, _, _, _) => methods.push(Box::new(func)),
                    _ => panic!("Tried to add something that wasn't a function expr to a class method block")
                }
                len = self.tree.len();
            }
        }
        // Push this to the tree without reducing (sketchy)
        self.tree.push(Expr::ClassMethods(Box::new(ident), methods));
        self.state = State::Empty;
    }

    fn expr_class_expr(&mut self) {
        let mut fields = Vec::<Box<Expr>>::new();
        let mut last_token_was_comma = true;
        while let Some(token) = self.tokens.next() {
            match token.0 {
                TokenKind::RCurl => {
                    if !last_token_was_comma {
                        let expr = self.stack.pop().unwrap_or_else(|| {
                            panic!("Expected a valid expr for struct fields");
                        });
                        fields.push(Box::new(expr));
                    }
                    break;
                }
                TokenKind::Comma => {
                    dbg!(&self.stack);
                    last_token_was_comma = true;
                    let expr = self.stack.pop().unwrap_or_else(|| {
                        panic!("Expected a valid expr for struct field");
                    });
                    fields.push(Box::new(expr));
                }
                _ => {
                    self.parse_expr(token);
                    last_token_was_comma = false;
                }
            }
        }
        self.stack.push(Expr::ClassFields(fields));
        self.try_reduce();
    }

    fn expr_import_args(&mut self) {
        let mut args = Vec::<Box<Expr>>::new();
        while let Some(token) = self.tokens.next() {
            dbg!(&self.stack);
            dbg!(&args);
            match token.0 {
                TokenKind::SemiColon => {
                    let expr = self.stack.pop().unwrap_or_else(|| {
                        panic!("Expected a valid expr for import arg");
                    });
                    args.push(Box::new(expr));
                    if args.is_empty() {
                        panic!("Expected symbols to import from module!");
                    }
                    break;
                }
                TokenKind::Comma => {
                    let expr = self.stack.pop().unwrap_or_else(|| {
                        panic!("Expected a valid expr for import arg");
                    });
                    args.push(Box::new(expr));
                }
                _ => self.parse_expr(token),
            }
        }
        self.stack.push(Expr::ImportArgs(args));
        self.try_reduce();
    }

    fn expr_function_call(&mut self) {
        let args = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid params for function call");
        });
        let ident = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid identifier for function call");
        });
        self.stack
            .push(Expr::FunctionCall(Box::new(ident), Box::new(args)));
    }

    fn expr_arguments(&mut self) {
        let mut args = Vec::<Box<Expr>>::new();
        while let Some(token) = self.tokens.next() {
            match token.0 {
                TokenKind::RPar => {
                    if !args.is_empty() {
                        let expr = self.stack.pop().unwrap_or_else(|| {
                            panic!("Expected a valid expr for parameter");
                        });
                        args.push(Box::new(expr));
                    }
                    break;
                }
                TokenKind::Comma => {
                    let expr = self.stack.pop().unwrap_or_else(|| {
                        panic!("Expected a valid expr for parameter");
                    });
                    args.push(Box::new(expr));
                }
                _ => self.parse_expr(token),
            }
        }
        self.stack.push(Expr::FunctionArgs(args));
    }

    fn expr_parameter(&mut self) {
        let ident = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid LHS identifier for parameter");
        });

        if let Some(token) = self.tokens.next() {
            self.parse_expr(token);
            // Make sure we keep parsing just in case the identifier is qualified
            while let Some(token) = self.tokens.peek() {
                if token.0 != TokenKind::Dot {
                    break;
                }
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
        let mut params = Vec::<Box<Expr>>::new();
        let mut empty_params = true;
        while let Some(token) = self.tokens.next() {
            match token.0 {
                TokenKind::RPar => {
                    if !empty_params {
                        let expr = self.stack.pop().unwrap_or_else(|| {
                            panic!("Expected a valid expr for parameter");
                        });
                        params.push(Box::new(expr));
                    }
                    break;
                }
                TokenKind::Comma => {
                    let expr = self.stack.pop().unwrap_or_else(|| {
                        panic!("Expected a valid expr for parameter");
                    });
                    params.push(Box::new(expr));
                }
                _ => {
                    self.parse_expr(token);
                    empty_params = false;
                }
            }
        }
        self.stack.push(Expr::FunctionArgs(params));
        self.state = State::PostParamFunctionExpr;
    }

    fn expr_parens(&mut self) {
        while let Some(token) = self.tokens.next() {
            if token.0 == TokenKind::RPar {
                break;
            }
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
            if token.0 == TokenKind::RCurl {
                break;
            }
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
        match self.state {
            State::PostParamFunctionExpr => self.try_reduce(),
            _ => {}
        }
    }

    fn expr_qmark(&mut self) {
        let ident = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid LHS expression for QMark");
        });
        self.stack.push(Expr::QMark(Box::new(ident)));
    }

    fn expr_binaryop(&mut self, operator: BinaryOperator) {
        let left = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected a valid LHS expression for BinOp");
        });

        if let Some(token) = self.tokens.next() {
            self.parse_expr(token);
            // Make sure we keep parsing just in case an identifier is qualified
            while let Some(token) = self.tokens.peek() {
                if token.0 != TokenKind::Dot {
                    break;
                }
                let token = self.tokens.next().unwrap();
                self.parse_expr(token);
            }
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

    fn expr_str(&mut self, string: &'a str) {
        self.stack.push(Expr::Str(string.to_string()));
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
