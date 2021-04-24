pub struct InputStream {
    pos: usize,
    line: i64,
    col: i64,
    input: String,
}

impl InputStream {
    fn new(input: String) -> InputStream {
        InputStream {
            pos: 0,
            line: 1,
            col: 0,
            input: input,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().next()
    }

    fn next(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn is_eof(&self) -> bool {
        self.peek().is_none()
    }

    fn croak(&self, msg: &str) {
        panic!("{} ({}:{})", msg, self.line, self.col)
    }
}
