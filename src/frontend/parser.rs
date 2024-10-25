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
            if !current.is_branch_node() {
                let n = self.parse_expr();
                self.ast.push(n);
            }

            // Advance position
            self.pos += 1;
        }
    }

    fn parse_expr(&mut self) -> Node<'a> {
        match self.variants.as_slice()[self.pos..] {
            [&TokenKind::Ident(_), ..] => self.parse_ident(false),
            [&TokenKind::EOF, ..] => self.parse_eof(),
            _ => panic!("Unexpected token"),
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
                    return Node::Ident(s);
                } else {
                    // Parsing left to right
                    println!("{}", s);
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

    fn lookahead_for(&mut self, n: isize, k: TokenKind) -> bool {
        let t = self.variants.get((self.pos as isize + n) as usize);
        if t.is_some_and(|t| t == &&k) {
            return true;
        } else {
            return false;
        }
    } 
}