pub enum Token {
    Punc { value: String },
    Num { value: f64 },
    Str { value: String },
    Kw { value: String },
    Var { value: String },
    Op { value: String },
    Error,
}
