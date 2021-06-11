#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Lambda {
        vars: Vec<Expr>,
        body: Box<Expr>,
    },
    Block {
        exprs: Vec<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        otherwise: Option<Box<Expr>>,
    },
    Var {
        name: String,
    },
    Bool {
        value: bool,
    },
    Str {
        value: String,
    },
    Num {
        value: f64,
    },
    Assign {
        operator: String, // remove this
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Binary {
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Error,
}

impl Expr {
    pub fn name(&self) -> String {
        match self {
            Expr::Lambda { .. } => "lambda".into(),
            Expr::Block { .. } => "block".into(),
            Expr::Call { .. } => "call".into(),
            Expr::If { .. } => "if".into(),
            Expr::Var { .. } => "variable".into(),
            Expr::Bool { .. } => "boolean".into(),
            Expr::Str { .. } => "string".into(),
            Expr::Num { .. } => "number".into(),
            Expr::Assign { .. } => "assign".into(),
            Expr::Binary { .. } => "binary".into(),
            Expr::Error => "error".into(),
        }
    }
}
