struct Scanner {
    chars: Vec<char>,
    offset: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            chars: vec![],
            offset: 0,
        }
    }
}

impl Scanner {
    /// Peeks one character ahead
    fn peek(&self, offset: Option<usize>) -> char {
        self.chars[self.offset + offset.unwrap_or(0)]
    }

    /// Peeks one characher ahead and consumes the value
    fn consume(&mut self) -> char {
        let consumed_char = self.chars[self.offset];
        self.offset += 1;
        consumed_char
    }
}

pub fn parse(input: String) -> String {
    loop {}
}
