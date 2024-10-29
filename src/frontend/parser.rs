use super::{node::Node, token::{Token, TokenKind}};

/// Parse method uses state-based parsing just without the tables
/// State 0: Default state
/// State 1: Looking for identifier to form a qualified identifier
///     Indicator: `.`
pub fn parse<'a>(stream: &'a Vec<Token>) -> Option<Vec<Node<'a>>> {
    let mut stack = Vec::<Node<'a>>::new();
    let mut state = 0usize;
    
    // Shadow stream to make it a peekable iter
    let mut stream = stream.into_iter().peekable();

    while let Some(token) = stream.next() {
        match token {
            Token(TokenKind::Ident(name), ..) => {
                match state {
                    0 => stack.push(Node::Ident(name)),
                    1 => {
                        let right = Box::new(Node::Ident(name));
                        let left = match stack.pop() {
                            Some(n) => Box::new(n),
                            None => panic!("Expected identifier in state 1"),
                        };
                        stack.push(Node::QIdent(left, right));
                    }
                    _ => panic!("Got identifier in parser state {}", state),
                }
            },
            Token(TokenKind::Dot, ..) => state = 1, // shift to state 1
            Token(TokenKind::EOF, ..) => break,
            _ => {}
        }
    }
    Some(stack)
}