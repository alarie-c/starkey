use std::ops::Range;

#[derive(Debug)]
pub struct Errors(Vec<SkError>);

impl Errors {
    pub fn initialize() -> Self {
        Self(Vec::new())
    }

    /// Creates a new error an pushes it to the Errors vec
    /// Start and end are inclusive such that the range is start..=end
    pub fn new(&mut self, class: ErrorClass, kind: ErrorKind, start: usize, end: usize) {
        self.0.push(SkError {
            class,
            kind,
            span: start..end + 1,
        })
    }

    pub fn dbg(&self) {
        self.0.iter().for_each(|x| println!("{:?}", x));
    }
}

#[derive(Debug)]
pub enum ErrorClass {
    Warning,
    Error,
}

#[derive(Debug)]
pub enum ErrorKind {
    ParseError(String),
}

#[derive(Debug)]
pub struct SkError {
    pub class: ErrorClass,
    pub kind: ErrorKind,
    pub span: Range<usize>,
}
