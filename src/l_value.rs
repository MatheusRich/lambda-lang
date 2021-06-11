#[derive(Clone, PartialEq, Debug)]
pub enum LValue {
    Str(String),
    Num(f64),
    Bool(bool),
    // Lambda(bool),
}

impl From<f64> for LValue {
    fn from(item: f64) -> Self {
        LValue::Num(item)
    }
}

impl From<String> for LValue {
    fn from(item: String) -> Self {
        LValue::Str(item)
    }
}

impl From<bool> for LValue {
    fn from(item: bool) -> Self {
        LValue::Bool(item)
    }
}
