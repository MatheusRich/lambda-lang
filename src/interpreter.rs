use super::{Env, Expr, LValue};

pub fn evaluate(expr: Expr, env: &mut Env) -> Result<LValue, String> {
    match expr {
        Expr::Num { value } => Ok(value.into()),
        Expr::Str { value } => Ok(value.into()),
        Expr::Bool { value } => Ok(value.into()),
        Expr::Var { name } => env.get(name),
        Expr::Assign { left, right, .. } => match *left {
            Expr::Var { name } => {
                let rhs = evaluate(*right, env)?;
                env.set(&name, &rhs)
            }
            _ => Err(format!("cannot assign to {}", left.name())),
        },
        Expr::Error => {
            Err("Internal interpreter error: don't know how to evaluate error expression".into())
        }
        _ => Err(format!("Don't know how to evaluate expression {:?}", expr)),
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate, Env, Expr, LValue};

    #[test]
    fn it_evaluates_a_number() {
        let input = Expr::Num { value: 1.0 };
        let mut env = Env::new();

        let result = evaluate(input, &mut env);

        assert!(result.is_ok());
        assert_eq!(LValue::Num(1.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_string() {
        let input = Expr::Str {
            value: "Rusty!".into(),
        };
        let mut env = Env::new();

        let result = evaluate(input, &mut env);

        assert!(result.is_ok());
        assert_eq!(LValue::Str("Rusty!".into()), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_boolean() {
        let input = Expr::Bool { value: true };
        let mut env = Env::new();

        let result = evaluate(input, &mut env);

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_var_expr() {
        let input = Expr::Var {
            name: "my_bool".into(),
        };
        let mut env = Env::new();
        env.def("my_bool".into(), &LValue::Bool(true));

        let result = evaluate(input, &mut env);

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());
    }

    #[test]
    fn it_fails_to_evaluate_an_undefined_variable() {
        let input = Expr::Var {
            name: "undefined_var".into(),
        };
        let mut env = Env::new();

        let result = evaluate(input, &mut env);

        assert!(result.is_err());
        assert_eq!(
            String::from("undefined variable undefined_var"),
            result.unwrap_err()
        );
    }

    #[test]
    fn it_evaluates_an_assign_expr() {
        let input = Expr::Assign {
            operator: "=".into(),
            left: Box::new(Expr::Var {
                name: "my_bool".into(),
            }),
            right: Box::new(Expr::Bool { value: true }),
        };
        let mut env = Env::new();
        env.def("my_bool".into(), &LValue::Bool(false));

        let result = evaluate(input, &mut env);

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());
    }

    #[test]
    fn it_deny_assigning_to_expr_that_is_not_a_variable() {
        let input = Expr::Assign {
            operator: "=".into(),
            left: Box::new(Expr::Str {
                value: "my_bool".into(),
            }),
            right: Box::new(Expr::Bool { value: true }),
        };
        let mut env = Env::new();
        env.def("my_bool".into(), &LValue::Bool(false));

        let result = evaluate(input, &mut env);

        assert!(result.is_err());
        assert_eq!(
            String::from("cannot assign to string expression"),
            result.unwrap_err()
        );
    }

    #[test]
    fn it_does_not_evaluate_an_error_expr() {
        let input = Expr::Error;
        let mut env = Env::new();

        let result = evaluate(input, &mut env);

        assert!(result.is_err());
        assert_eq!(
            String::from("Internal interpreter error: don't know how to evaluate error expression"),
            result.unwrap_err()
        );
    }
}
