use crate::{node::Node, token::{Token, TokenKind}};

pub struct Parser<'a> {
    variants: Vec<&'a TokenKind>,
    tokens: &'a Vec<Token>,
    pos: usize,
    ast: Vec<Node>,
}

impl<'a> Parser<'a> {
    pub fn new(stream: &'a Vec<Token>) -> Self {
        Self {
            variants: stream.iter().map(|t| &t.kind).collect(),
            tokens: stream,
            pos: 0usize,
            ast: Vec::new(),
        }
    }

    pub fn parse_stream(&mut self) -> Vec<Node> {
        let mut ast = Vec::<Node>::new();
        
        loop {
            // Check for EOF condition
            if &self.pos >= &self.tokens.len() {
                break;
            }
            // Get current token
            let current = self.variants.get(self.pos).unwrap();

            // Attempt to parse an expression
            if !current.is_branch_node() {
                match self.parse_expr() {
                    Some(n) => ast.push(n),
                    None => {},
                }
            }

            // Advance position
            self.pos += 1;
        }

        // Return AST
        ast
    }

    fn parse_expr(&mut self) -> Option<Node> {
        match self.variants.as_slice()[self.pos..] {
            [TokenKind::Number(v), ..] => self.parse_number(v),
            [TokenKind::Str(v), ..] => self.parse_str(v),
            [TokenKind::Ident(n), ..] => self.parse_ident(n),

            [TokenKind::Let, ..] => self.parse_variable_assignment(),

            _ => None,
        }
    }

    fn parse_variable_assignment(&mut self) -> Option<Node> {
        
        if !self.advance() { todo!("Missing identifier or type error") }
        let ident_or_type = self.parse_expr().unwrap_or_else(|| {
            todo!("Parse ident for var assign error");
        });
        
        let type_or_equals = self.parse_expr().unwrap_or_else(|| {
            todo!("Parse ident for var assign error");
        });



        None
        
    }

    fn parse_number(&mut self, value: &String) -> Option<Node> {
        if value.contains(".") {
            // Floating point number literal
            let parsed: f32 = value.parse().unwrap_or_else(|_| {
                todo!("Float parse error");
            });
            Some(Node::Float(parsed))
        } else {
            // Floating point number literal
            let parsed: i32 = value.parse().unwrap_or_else(|_| {
                todo!("Int parse error");
            });
            Some(Node::Integer(parsed))
        }
    }

    fn parse_str(&mut self, value: &String) -> Option<Node> {
        Some(Node::Str(value.to_string()))
    }

    fn parse_ident(&mut self, name: &String) -> Option<Node> {
        // TODO: Add look left and look right logic
        Some(Node::Ident(name.to_string()))
    }

    fn advance(&mut self) -> bool {
        if self.pos >= self.tokens.len() {
            false
        } else {
            self.pos += 1;
            true
        }
    }
}
