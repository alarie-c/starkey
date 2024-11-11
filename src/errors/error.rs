use std::ops::Range;

use super::formatter::Formatter;

#[derive(Debug)]
pub struct Errors<'a> {
    errs: Vec<SkError>,
    fmt: &'a Formatter<'a>,
}

impl<'a> Errors<'a> {
    pub fn initialize(fmt: &'a Formatter<'a>) -> Self {
        Self {
            errs: Vec::new(),
            fmt,
        }
    }

    /// Creates a new error an pushes it to the Errors vec
    /// Start and end are inclusive such that the range is start..=end
    pub fn new(&mut self, class: ErrorClass, kind: ErrorKind, start: usize, end: usize) {
        self.errs.push(SkError {
            class,
            kind,
            span: start..end + 1,
        })
    }

    pub fn dbg(&self) {
        self.errs.iter().for_each(|x| println!("{:?}", x));
    }

    fn print_error(&self, err: &SkError) {
        // Get the line this belongs to
        let (line_str, range) = self.fmt.get_line(err.span.start);
        let underline = Formatter::get_underline(&line_str, range);
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
