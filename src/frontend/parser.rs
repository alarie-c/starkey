use super::{node::Node, token::{Token, TokenKind}};

pub struct Parser<'a> {
    pub ast: Vec<Node<'a>>,
    variants: Vec<&'a TokenKind<'a>>,
    tokens: &'a Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(stream: &'a Vec<Token<'a>>) -> Self {
        Self {
            ast: Vec::new(),
            variants: stream.iter().map(|t| &t.kind).collect(),
            tokens: stream,
            pos: 0usize,
        }
    }

    pub fn parse(&mut self) {
        loop {
            // Check for EOF condition
            if &self.pos >= &self.tokens.len() {
                break;
            }
            // Get current token
            let current = self.variants.get(self.pos).unwrap();

            // Attempt to parse an expression
            if current.is_branch_node() {
                let n = self.parse_expr();
                self.ast.push(n);
            }

            // Advance position
            self.pos += 1;
        }
    }

    fn parse_expr(&mut self) -> Node<'a> {
        dbg!(self.variants.get(self.pos));
        match self.variants.as_slice()[self.pos..] {
            [&TokenKind::New, &TokenKind::Var, &TokenKind::Ident(s), ..] => self.parse_new(s, true),
            [&TokenKind::New, &TokenKind::Ident(s), ..] => self.parse_new(s, false),
            [&TokenKind::Ident(_), ..] => self.parse_ident(false),
            [&TokenKind::Str(s), ..] => Node::Str(s), 
            [&TokenKind::EOF, ..] => self.parse_eof(),
            _ => panic!("Unexpected token"),
        }
    }

    /// Suprisingly simple
    /// We skip some tokens depending on what we matched in the slice from parse_expr()
    /// Then, we look for a colon to tell if there's a type annotation anywhere.
    /// 
    /// From there it's just a matter of moving ahead, parsing, then moving ahead again
    /// Will return Node::Var if var is true
    /// The rest of the members stay exactly the same though
    fn parse_new(&mut self, name: &'a str, var: bool) -> Node<'a> {
        let ident = Node::Ident(name);
        self.pos += 1; // we already know this exists from the slice in parse_expr()
        if var { self.pos += 1 } // ditto

        // Look for a type annotation
        if self.lookahead_for(1, TokenKind::Colon) {
            println!("Type annotation found");
            // Type annotation found
            self.pos += 1;
            
            // Get type identifier
            let typ = self.advance().then(|| {
                self.parse_expr()
            }).unwrap_or_else(|| {
                panic!("Expected a type identifier, got EOF");
            });

            // Skip the equal sign and get the value
            if !self.advance() { panic!("Expected an assignment operator, got EOF"); } // type identifier -> =
            // if !self.advance() { panic!("Expected a value, got EOF"); } // = -> value

            // Get the value of the variable
            let value = self.advance().then(|| {
                self.parse_expr()
            }).unwrap_or_else(|| {
                panic!("Expected a value after assignment, got EOF");
            });

            dbg!(&ident);
            dbg!(&value);
            dbg!(&typ);
            
            // Return the node
            if var {
                Node::Var { ident: Box::new(ident), value: Box::new(value), typ: Some(Box::new(typ)) }    
            } else {
                Node::New { ident: Box::new(ident), value: Box::new(value), typ: Some(Box::new(typ)) }    
            }
        } else {
            // No type annotation
            // Skip the equal sign and get the value
            if !self.advance() { panic!("Expected an assignment operator, got EOF"); } // name identifier -> =
            if !self.advance() { panic!("Expected a value, got EOF"); } // = -> value

            // Get the value of the variable
            let value = self.advance().then(|| {
                self.parse_expr()
            }).unwrap_or_else(|| {
                panic!("Expected a value after assignment, got EOF");
            });

            dbg!(&ident);
            dbg!(&value);

            // Return the node
            if var {
                Node::Var { ident: Box::new(ident), value: Box::new(value), typ: None }    
            } else {
                Node::New { ident: Box::new(ident), value: Box::new(value), typ: None }    
            }
        }
    }

    /// This function parses an Ident token and then looks to the left and right for 
    /// possible attempts to access members or methods
    /// 
    /// If the identifier is being read right to left, the ORIGINAL Node::AccessMember
    /// must be returned such that AccessMember { parent: Ident @ pos - 2, child: Ident @ pos }
    /// 
    /// If the identifier is being read from left to right, the most recent Node::AcessMember
    /// must be returned such that AccessMember { parent: Ident @ pos, child: Ident @ pos + 1 }
    fn parse_ident(&mut self, rl: bool) -> Node<'a> {
        // Start by getting the current thing
        match self.variants.get(self.pos).unwrap() {
            TokenKind::Ident(s) => {
                if rl {
                    // Parsing right to left
                    let current = Node::Ident(s);
                    if self.lookahead_for(-1, TokenKind::Dot) && self.pos - 2 > 0 {
                        self.pos -= 2;
                        let parent = self.parse_ident(true);
                        return Node::AccessMember { parent: Box::new(parent), child: Box::new(current) };
                    } else {
                        return current;
                    }
                } else {
                    // Parsing left to right
                    let current = Node::Ident(s);
                    if self.lookahead_for(1, TokenKind::Dot) && self.pos + 2 < self.variants.len() {
                        self.pos += 2;
                        let child = self.parse_ident(false);
                        return Node::AccessMember { parent: Box::new(current), child: Box::new(child) };
                    } else {
                        return current;
                    }
                }
            }
            _ => panic!("Expected identifier"),
        }
    }

    fn parse_eof(&mut self) -> Node<'a> {
        Node::Exit(0)
    }

    fn advance(&mut self) -> bool {
        if self.pos + 1 < self.variants.len() {
            self.pos += 1;
            true 
        } else {
            false
        }
    }

    fn lookahead_for(&mut self, n: isize, k: TokenKind) -> bool {
        let t = self.variants.get((self.pos as isize + n) as usize);
        if t.is_some_and(|t| t == &&k) {
            return true;
        } else {
            return false;
        }
    } 
}