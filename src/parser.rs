use crate::{
    node::{Node, VariableAssignment},
    token::{Token, TokenKind},
};

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
            if current.is_branch_node() {
                match self.parse_expr() {
                    Some(n) => ast.push(n),
                    None => {}
                }
            }

            // Advance position
            self.pos += 1;
        }

        // Return AST
        ast
    }

    /// Parses the current token regardless of if it is a branch node or not
    /// Called by functions that construct complicated nodes contained boxed nodes
    /// Will return None for unexpected tokens, its the calling function's job to deal
    /// with unexpected tokens as happen
    fn parse_expr(&mut self) -> Option<Node> {
        match self.variants.as_slice()[self.pos..] {
            [TokenKind::Number(v), ..] => self.parse_number(v),
            [TokenKind::Str(v), ..] => self.parse_str(v),
            [TokenKind::Ident(n), ..] => self.parse_ident(n),

            [TokenKind::Let, ..] => self.parse_variable_assignment(),

            _ => None,
        }
    }

    /// Parent Format (with annotation): `let <ident> : <ident> = <value>`
    ///
    /// Parent Format (without annotation): `let <ident> = <value>`
    /// Will return a node containing other nodes for the variable assignment
    fn parse_variable_assignment(&mut self) -> Option<Node> {
        // Get the name of the identifier
        let ident = self
            .advance()
            .then(|| {
                self.parse_expr().unwrap_or_else(|| {
                    todo!("recover invalid name");
                })
            })
            .unwrap();

        // Check for type annotation
        if !self.advance() {
            todo!("recover missing operator 1")
        }
        let next = self.variants.get(self.pos).unwrap();

        // Type annotation present
        if *next == &TokenKind::Colon {
            // Look for type and value
            let typ = self
                .advance()
                .then(|| {
                    self.parse_expr().unwrap_or_else(|| {
                        todo!("recover invalid type");
                    })
                })
                .unwrap();
            dbg!(&typ);
            // Skip the equal sign
            if !self.advance() {
                todo!("recover missing operator 2")
            }

            // Get the expression value
            let value = self
                .advance()
                .then(|| {
                    self.parse_expr().unwrap_or_else(|| {
                        todo!("recover invalid value");
                    })
                })
                .unwrap();

            // Return the node
            Some(Node::VariableAssignment(VariableAssignment {
                ident: Box::new(ident),
                constant: false,
                value: Box::new(value),
                typ: Some(Box::new(typ)),
            }))

        // Type annotation absent
        } else if *next == &TokenKind::Equal {
            // Get the expression value
            let value = self
                .advance()
                .then(|| {
                    self.parse_expr().unwrap_or_else(|| {
                        todo!("recover invalid value");
                    })
                })
                .unwrap();

            // Return the node
            Some(Node::VariableAssignment(VariableAssignment {
                ident: Box::new(ident),
                constant: false,
                value: Box::new(value),
                typ: None,
            }))

        // Got something other than = or :
        } else {
            todo!("invalid operator")
        }
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

    /// Attempts to advance the position of the parser
    /// If successful, will return Ok(bool) based on if the variant matches provided variant
    /// If unsuccessful, will return Err(())
    fn assert_next(&mut self, kind: &TokenKind) -> Result<bool, ()> {
        // Look for the next token and
        self.pos += 1;
        match self.variants.get(self.pos) {
            Some(t) => {
                if *t == kind {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Err(()),
        }
    }

    // Advances the parser's position and returns true or false
    // True -> success
    // False -> EOF condition reached
    fn advance(&mut self) -> bool {
        if self.pos >= self.tokens.len() {
            false
        } else {
            self.pos += 1;
            true
        }
    }
}
