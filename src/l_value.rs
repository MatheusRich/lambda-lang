use super::{evaluate, Env, Expr};

#[derive(Clone, PartialEq, Debug)]
pub enum LValue {
    Str(String),
    Num(f64),
    Bool(bool),
    Lambda(Lambda),
    Lambda2(Box<fn(Vec<LValue>) -> LValue>),
}

impl LValue {
    pub fn name(&self) -> &str {
        match self {
            LValue::Str(_) => "string",
            LValue::Num(_) => "number",
            LValue::Bool(_) => "boolean",
            LValue::Lambda(_) => "lambda",
            LValue::Lambda2(_) => "lambda2",
        }
    }
}

impl std::fmt::Display for LValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match self {
            LValue::Str(string) => string.clone(),
            LValue::Num(num) => num.to_string(),
            LValue::Bool(boolean) => boolean.to_string(),
            LValue::Lambda(_) => "lambda".to_string(),
            LValue::Lambda2(_fun) => "lambda()".to_string(),
        };

        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Lambda {
    pub vars: Vec<String>,
    pub body: Expr,
    pub env: Env,
}

impl Lambda {
    pub fn call(&self, args: Vec<LValue>) -> Result<LValue, String> {
        if args.len() < self.vars.len() {
            return Err(format!(
                "too few arguments (given {}, expected {})",
                args.len(),
                self.vars.len()
            ));
        }

        let mut scope = Env::with_enclosing(self.env.clone());

        for (i, var) in self.vars.iter().enumerate() {
            scope.def(var.clone(), &args[i]);
        }

        evaluate(self.body.clone(), &mut scope)
    }
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
