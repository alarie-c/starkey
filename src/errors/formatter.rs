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
        lines.iter().for_each(|x| buffer.push_str((self.lines[*x].to_owned() + "\n").as_str()));
        return buffer;
    }
}