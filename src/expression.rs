#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Lambda {
        vars: Vec<Expression>,
        body: Box<Expression>,
    },
    Prog {
        prog: Vec<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    If {
        cond: Box<Expression>,
        then: Box<Expression>,
        otherwise: Option<Box<Expression>>,
    },
    Var {
        name: String,
    },
    Bool {
        value: bool,
    },
    String {
        value: String,
    },
    Num {
        value: f64,
    },
    Assign {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Binary {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Error,
}
