use super::InputStream;
use super::Token;

pub struct TokenStream {
    keywords: Vec<String>,
    current: Option<Token>,
    input: InputStream,
}

impl TokenStream {
    pub fn new(input: InputStream) -> TokenStream {
        TokenStream {
            keywords: "if then else lambda λ true false"
                .split(" ")
                .map(str::to_string)
                .collect(),
            current: None,
            input: input,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let current = self.current.clone(); // TODO: I think I can move here
        self.current = None;

        match current {
            Some(token) => Some(token),
            None => {
                let token = self.read_next();

                token
            }
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.current.is_none() {
            self.current = self.read_next();
        }

        self.current.clone()
    }

    pub fn is_eof(&mut self) -> bool {
        return self.peek().is_none();
    }

    pub fn croak(&self, msg: &str) {
        self.input.croak(msg)
    }

    fn read_next(&mut self) -> Option<Token> {
        self.read_while(is_whitespace);

        let ch: char;

        match self.input.peek() {
            Some(c) => ch = c,
            None => return None,
        }

        if ch == '#' {
            self.skip_comment();
            return self.read_next();
        }

        if ch == '"' {
            return Some(self.read_string());
        }

        if is_digit(&ch) {
            return Some(self.read_number());
        }

        if is_id_start(&ch) {
            return Some(self.read_identifier());
        }

        if is_punc(&ch) {
            return Some(Token::Punc {
                value: String::from(self.input.next()?),
            });
        }

        if is_op_char(&ch) {
            return Some(Token::Op {
                value: self.read_while(is_op_char),
            });
        }

        self.syntax_error(&format!("Can't handle character: {:?}", ch));

        None
    }

    fn skip_comment(&mut self) {
        self.read_while(|c| *c != '\n');
        self.input.next(); // reads the '\n' in the end
    }

    fn read_string(&mut self) -> Token {
        self.input.next(); // reads the quote
        let mut escaped = false;
        let mut string = String::from("");

        while let Some(c) = self.input.next() {
            if escaped {
                string.push(c);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                return Token::Str { value: string };
            } else {
                string.push(c);
            }
        }

        self.syntax_error("Unterminated string");

        Token::Error
    }

    fn read_number(&mut self) -> Token {
        let mut number = self.read_while(is_digit);

        if let Some(c) = self.input.peek() {
            if c == '.' {
                number.push('.');
                self.input.next();
                number += &self.read_while(is_digit);
            }
        }

        Token::Num {
            value: number.parse().unwrap(),
        }
    }

    fn read_identifier(&mut self) -> Token {
        let id = self.read_while(is_id);

        if self.is_keyword(&id) {
            Token::Kw { value: id }
        } else {
            Token::Var { value: id }
        }
    }

    fn read_while(&mut self, test: impl FnMut(&char) -> bool) -> String {
        self.input.read_while(test)
    }

    fn syntax_error(&self, msg: &str) {
        self.input.croak(&format!("SYNTAX ERROR: {}", msg));
    }

    fn is_keyword(&self, id: &String) -> bool {
        self.keywords.contains(id)
    }
}

fn is_whitespace(c: &char) -> bool {
    c.is_ascii_whitespace()
}

fn is_punc(c: &char) -> bool {
    ",;(){}[]".contains(*c)
}

fn is_op_char(c: &char) -> bool {
    "+-*/%=&|<>!".contains(*c)
}

fn is_id_start(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_' || *c == 'λ'
}

fn is_digit(c: &char) -> bool {
    c.is_ascii_digit()
}

fn is_id(c: &char) -> bool {
    is_id_start(c) || "?!-<>=0123456789".contains(*c)
}
