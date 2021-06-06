use std::fmt;

#[derive(Clone, Debug)]
pub enum Token {
    Kw { value: String },
    Num { value: f64 },
    Op { value: String },
    Punc { value: String },
    Str { value: String },
    Var { value: String },
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String = match self {
            Token::Kw { value }
            | Token::Op { value }
            | Token::Punc { value }
            | Token::Str { value }
            | Token::Var { value } => value.to_string(),
            Token::Num { value } => value.to_string(),
            _ => String::from("ERROR"),
        };

        write!(f, "{}", result)
    }
}
