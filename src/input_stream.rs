pub struct InputStream {
    pos: u64,
    line: u64,
    col: u64,
    input: String,
}

impl InputStream {
    pub fn new(input: String) -> InputStream {
        InputStream {
            pos: 0,
            line: 1,
            col: 1,
            input,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos as usize)
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.pos as usize);
        self.pos += 1;

        if let Some(c) = c {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }

        c
    }

    pub fn read_while(&mut self, mut test: impl FnMut(&char) -> bool) -> String {
        let mut str = String::new();

        while let Some(c) = self.peek() {
            if test(&c) {
                self.next();
                str.push(c);
            } else {
                return str;
            }
        }

        str
    }

    pub fn croak(&self, msg: &str) {
        panic!("{} at line {}, col {}", msg, self.line, self.col);
    }
}
