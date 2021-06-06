#[derive(Clone, Debug)]
pub enum Expression {
    Lambda {
        vars: Vec<Box<Expression>>,
        body: Box<Expression>,
    },
    Prog {
        prog: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Box<Expression>>,
    },
    If {
        cond: Box<Expression>,
        then: Box<Expression>,
        r#else: Box<Expression>,
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
