use std::ops::Range;

#[derive(Debug)]
pub struct Formatter<'a> {
    source: &'a String,
    lines: Vec<&'a str>,
}

impl<'a> Formatter<'a> {
    pub fn initialize(source: &'a String) -> Self {
        let lines: Vec<&'a str> = source.lines().collect();
        Self { source, lines }
    }

    fn get_relevant_lines(&self, lines: Vec<usize>) -> String {
        let mut buffer = String::new();
        lines
            .iter()
            .for_each(|x| buffer.push_str((self.lines[*x].to_owned() + "\n").as_str()));
        return buffer;
    }

    pub fn get_line(&self, mut index: usize) -> (String, Range<usize>) {
        let mut buffer = String::new();
        let start: usize;
        let end: usize;

        // Get to the beginning of this line
        while let Some(c) = self.source.get(index.saturating_sub(1)..index) {
            if index == 0 {
                break;
            }

            match c {
                "\r" | "\n" => break,
                _ => index = index.saturating_sub(1),
            }
        }
        start = index;

        // Take until the end
        while let Some(c) = self.source.get(index..index + 1) {
            match c {
                "\r" | "\n" => break,
                _ => {
                    buffer.push_str(c);
                    index += 1;
                }
            }
        }
        end = index;

        return (buffer, start..end + 1);
    }

    /// Returns a string that underlines the desired range with `^`
    pub fn get_underline(line: &String, range: Range<usize>) -> String {
        let mut buffer = String::new();
        let chars = line.char_indices();
        for (i, _) in chars {
            if range.contains(&i) {
                buffer.push('^');
            } else {
                buffer.push(' ');
            }
        }

        return buffer;
    }
}
