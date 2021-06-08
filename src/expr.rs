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
        operator: String,
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
