// #[derive(Debug)]
// pub enum Node<'a> {
//     AccessMember {
//         parent: Box<Self>,
//         child: Box<Self>,
//     },

//     // Values
//     Float(f32),
//     Integer(i32),
//     Str(&'a str),
//     Ident(&'a str),

//     // Variables & Values
//     Var {
//         ident: Box<Self>,
//         value: Box<Self>,
//         typ: Option<Box<Self>>,
//     },
//     New {
//         ident: Box<Self>,
//         value: Box<Self>,
//         typ: Option<Box<Self>>,
//     },
//     Set {
//         ident: Box<Self>,
//         value: Box<Self>,
//     },

//     // Other
//     Exit(i32),
// }

#[derive(Debug)]
pub enum Atom<'a> {
    /// Ident atom is technically not atomic because it
    /// encapsulates indexing into an identifier.
    ///
    /// This is just to make it simpler for the rest
    /// of the parser to work, at leat for now.
    ///
    /// The index keys are listed in sequential order in `members`
    /// For example, `class.member.method` would become
    /// `Ident { name: "class", members: vec!["member", "method"]`
    Ident {
        name: &'a str,
        members: Vec<&'a str>,
    },

    Type {
        name: Box<Atom<'a>>,
    },

    Var,
    Const,

    Str(&'a str),
    Float(f32),
    Integer(i32),
}

#[derive(Debug)]
pub enum Terminal<'a> {
    Var { ident: &'a str, value: &'a Atom<'a> },
    Const { ident: &'a str, value: &'a Atom<'a> },
    Exit { code: i32 },
}
