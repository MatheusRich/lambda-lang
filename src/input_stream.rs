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
            input: input,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos as usize)
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.pos as usize);
        self.pos += 1;

        match c {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.col = 0;
                } else {
                    self.col += 1;
                }
            }
            _ => {}
        }

        c
    }

    pub fn read_while(&mut self, test: fn(&char) -> bool) -> String {
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

    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
    }

    pub fn croak(&self, msg: &str) {
        println!("{} at line {}, col {}", msg, self.line, self.col);

        std::process::exit(-1);
    }
}
