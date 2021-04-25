use super::InputStream;
use super::Token;

pub struct TokenStream {
    keywords: String,
    current: Option<Token>,
    input: InputStream,
}

impl TokenStream {
    pub fn new(input: InputStream) -> TokenStream {
        TokenStream {
            keywords: String::from("if then else lambda λ true false"),
            current: None,
            input: input,
        }
    }

    pub fn read_next(&mut self) -> Option<Token> {
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

        if is_digit(&ch) {
            return Some(self.read_number());
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

    fn read_number(&mut self) -> Token {
        let mut has_dot = false;
        let number: String = self.read_while(|c| {
            if *c == '.' {
                if has_dot {
                    return false;
                }

                has_dot = true;

                return true;
            }

            is_digit(c)
        });

        Token::Num {
            value: number.parse().unwrap(),
        }
    }

    fn read_while(&mut self, test: impl FnMut(&char) -> bool) -> String {
        self.input.read_while(test)
    }

    fn syntax_error(&self, msg: &str) {
        self.input.croak(&format!("SYNTAX ERROR: {}", msg));
    }
}

pub fn is_whitespace(c: &char) -> bool {
    c.is_ascii_whitespace()
}

fn is_punc(c: &char) -> bool {
    ",;(){}[]".contains(c.clone())
}

fn is_op_char(c: &char) -> bool {
    "+-*/%=&|<>!".contains(c.clone())
}

fn is_digit(c: &char) -> bool {
    c.is_ascii_digit()
}
