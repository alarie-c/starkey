use std::cell::RefCell;

use super::{
    node::{Atom, Terminal},
    token::{Token, TokenKind},
};

pub struct AST<'a> {
    pub nodes: Vec<Terminal<'a>>,
}

impl<'a> AST<'a> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

pub fn parse<'a>(stream: &'a Vec<Token<'a>>) {
    let tokens = stream.as_slice();
}

// pub struct Parser<'a> {
//     pub stream: &'a Vec<Token<'a>>,
//     pub pos: usize,
// }

// impl<'a> Parser<'a> {
//     pub fn new(stream: &'a Vec<Token<'a>>) -> Self {
//         Self {
//             stream,
//             pos: 0usize,
//         }
//     }

//     pub fn parse(&mut self) -> AST {
//         let mut ast = AST::new();

//         while self.pos < self.stream.len() {
//             let atoms = self.consume();
//             dbg!(&atoms);

//             match atoms.as_slice() {
//                 [Atom::Var, Atom::Ident {
//                     name: n,
//                     members: m,
//                 }] => {
//                     // Get the variable's value
//                     self.pos += 1;
//                     let value = self.consume();
//                     if m == &Vec::<&'a str>::new() {
//                         panic!("Cannot declare a variable for a member");
//                     }
//                 }
//                 _ => {}
//             }

//             self.pos += 1;
//         }

//         ast
//     }

//     fn consume(&mut self) -> Vec<Atom> {
//         let mut atoms = Vec::<Atom>::new();

//         while let Some(current) = self.stream.get(self.pos) {
//             match current {
//                 Token(TokenKind::Var, ..) => {
//                     atoms.push(Atom::Var);
//                     self.pos += 1;
//                 }
//                 Token(TokenKind::Equal, ..) => break,
//                 Token(TokenKind::Colon, ..) => {
//                     self.pos += 1;
//                     match self.stream.get(self.pos) {
//                         Some(t) => match t {
//                             Token(TokenKind::Ident(name), ..) => {
//                                 let mut members = Vec::<&'a str>::new();

//                                 // Lookahead for more identifiers, incrementing `pos` locally
//                                 self.pos += 1;
//                                 while let Some(Token(TokenKind::Ident(n), ..)) =
//                                     self.stream.get(self.pos)
//                                 {
//                                     members.push(n);
//                                     self.pos += 1;
//                                 }

//                                 atoms.push(Atom::Type {
//                                     name: Box::new(Atom::Ident { name, members }),
//                                 });
//                             }
//                             _ => panic!("Expected identifier after type annotation"),
//                         },
//                         None => {}
//                     }
//                 }
//                 Token(TokenKind::Ident(name), ..) => {
//                     let mut members = Vec::<&'a str>::new();

//                     // Lookahead for more identifiers, incrementing `pos` locally
//                     self.pos += 1;
//                     while let Some(Token(TokenKind::Ident(n), ..)) = self.stream.get(self.pos) {
//                         members.push(n);
//                         self.pos += 1;
//                     }

//                     atoms.push(Atom::Ident { name, members });
//                 }
//                 Token(TokenKind::Str(value), ..) => {
//                     atoms.push(Atom::Str(value));
//                     self.pos += 1;
//                 }
//                 Token(TokenKind::SemiColon, ..) | Token(TokenKind::EOF, ..) => break,
//                 _ => panic!("Unexpected token"),
//             };
//         }

//         atoms
//     }
// }
