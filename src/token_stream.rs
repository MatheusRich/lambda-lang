use super::input_stream::InputStream;
use super::token::Token;

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

        if is_punc(&ch) {
            return Some(Token::Punc {
                value: String::from(self.input.next()?),
            });
        }

        self.input.croak(&format!("Can't handle character: {:?}", ch));

        None
    }

    fn read_while(&mut self, test: fn(&char) -> bool) -> String {
        self.input.read_while(test)
    }
}

pub fn is_whitespace(c: &char) -> bool {
    c.is_ascii_whitespace()
}

fn is_punc(c: &char) -> bool {
    ",;(){}[]".contains(c.clone())
}
