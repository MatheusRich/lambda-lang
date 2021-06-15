use super::{Env, Expr, LValue, Lambda};

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
        Expr::Binary {
            left,
            right,
            operator,
        } => match operator.as_str() {
            "+" | "-" | "*" | "/" | "%" | ">" | "<" | "<=" | ">=" => {
                apply_numeric_op(operator.as_str(), *left, *right, env)
            }
            "&&" | "||" => apply_logical_op(operator.as_str(), *left, *right, env),
            "==" | "!=" => apply_equality_op(operator.as_str(), *left, *right, env),
            _ => Err(format!("cannot apply binary operator {}", operator)),
        },
        Expr::If {
            cond,
            then,
            otherwise,
        } => {
            let cond = evaluate(*cond, env)?;

            match cond {
                LValue::Bool(false) => match otherwise {
                    Some(else_branch) => evaluate(*else_branch, env),
                    None => Ok(LValue::Bool(false)),
                },
                _ => evaluate(*then, env),
            }
        }
        Expr::Lambda { vars, body } => Ok(LValue::Lambda(Lambda {
            body: *body,
            env: env.clone(),
            vars: {
                let mut strings = vec![];

                for var in vars {
                    if let Expr::Str { value } = var {
                        strings.push(value);
                    }
                }

                strings
            },
        })),
        Expr::Call { func, args } => {
            let lambda = evaluate(*func, env)?;

            match lambda {
                LValue::Lambda(a_lambda) => {
                    let mut evaluated_args = vec![];

                    for arg in args {
                        evaluated_args.push(evaluate(arg, env)?);
                    }

                    a_lambda.call(evaluated_args)
                }
                _ => Err(format!("Unable to call on {:?}", lambda.name())),
            }
        }
        Expr::Block { exprs } => {
            let mut result = LValue::Bool(false);

            for expr in exprs {
                result = evaluate(expr, env)?;
            }

            Ok(result)
        }
        Expr::Error => {
            Err("Internal interpreter error: don't know how to evaluate error expression".into())
        }
    }
}

fn apply_numeric_op(
    operator: &str,
    left: Expr,
    right: Expr,
    env: &mut Env,
) -> Result<LValue, String> {
    let lhs = evaluate(left.clone(), env)?;
    let rhs = evaluate(right.clone(), env)?;

    match (&lhs, &rhs) {
        (LValue::Num(a), LValue::Num(b)) => match operator {
            "+" => Ok(LValue::Num(a + b)),
            "-" => Ok(LValue::Num(a - b)),
            "*" => Ok(LValue::Num(a * b)),
            "/" => Ok(LValue::Num(a / b)),
            "%" => Ok(LValue::Num(a % b)),
            "<" => Ok(LValue::Bool(a < b)),
            ">" => Ok(LValue::Bool(a > b)),
            "<=" => Ok(LValue::Bool(a <= b)),
            ">=" => Ok(LValue::Bool(a >= b)),
            _ => Err(format!("cannot apply numeric operator {}", operator)),
        },
        _ => Err(format!(
            "expected two numbers, got {} {} {}",
            lhs.name(),
            operator,
            rhs.name()
        )),
    }
}

fn apply_logical_op(
    operator: &str,
    left: Expr,
    right: Expr,
    env: &mut Env,
) -> Result<LValue, String> {
    let lhs = evaluate(left.clone(), env)?;

    match operator {
        "&&" => match lhs {
            LValue::Bool(false) => Ok(LValue::Bool(false)),
            _ => evaluate(right, env),
        },
        "||" => match lhs {
            LValue::Bool(false) => evaluate(right, env),
            _ => Ok(lhs),
        },
        _ => Err(format!("cannot apply logical operator {}", operator)),
    }
}

fn apply_equality_op(
    operator: &str,
    left: Expr,
    right: Expr,
    env: &mut Env,
) -> Result<LValue, String> {
    let lhs = evaluate(left.clone(), env)?;
    let rhs = evaluate(right.clone(), env)?;

    match operator {
        "==" => Ok(LValue::Bool(lhs == rhs)),
        "!=" => Ok(LValue::Bool(lhs != rhs)),
        _ => Err(format!("cannot apply equality operator {}", operator)),
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
    fn it_denies_assigning_to_expr_that_is_not_a_variable() {
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
        assert_eq!(String::from("cannot assign to string"), result.unwrap_err());
    }

    #[test]
    fn it_evaluates_a_sum() {
        let input = Expr::Binary {
            operator: "+".into(),
            left: Box::new(Expr::Num { value: 1.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(3.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_sum_with_variables() {
        let input = Expr::Binary {
            operator: "+".into(),
            left: Box::new(Expr::Var { name: "a".into() }),
            right: Box::new(Expr::Var { name: "b".into() }),
        };

        let mut env = Env::new();
        env.set("a", &LValue::Num(1.0)).unwrap();
        env.set("b", &LValue::Num(2.0)).unwrap();

        let result = evaluate(input, &mut env);

        assert!(result.is_ok());
        assert_eq!(LValue::Num(3.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_subtraction() {
        let input = Expr::Binary {
            operator: "-".into(),
            left: Box::new(Expr::Num { value: 1.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(-1.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_multiplication() {
        let input = Expr::Binary {
            operator: "*".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(4.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_division() {
        let input = Expr::Binary {
            operator: "/".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(1.0), result.unwrap());
    }

    #[test]
    fn it_does_not_error_on_a_division_by_zero() {
        let input = Expr::Binary {
            operator: "/".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 0.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(f64::INFINITY), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_mod_operation() {
        let input = Expr::Binary {
            operator: "%".into(),
            left: Box::new(Expr::Num { value: 5.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(1.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_greater_than() {
        let input = Expr::Binary {
            operator: ">".into(),
            left: Box::new(Expr::Num { value: 5.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());

        let input = Expr::Binary {
            operator: ">".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 5.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
    }

    #[test]
    fn it_evaluates_smaller_than() {
        let input = Expr::Binary {
            operator: "<".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 5.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());

        let input = Expr::Binary {
            operator: "<".into(),
            left: Box::new(Expr::Num { value: 5.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
    }

    #[test]
    fn it_evaluates_greater_or_equals_than() {
        let input = Expr::Binary {
            operator: ">=".into(),
            left: Box::new(Expr::Num { value: 5.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());

        let input = Expr::Binary {
            operator: ">=".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());
    }

    #[test]
    fn it_evaluates_smaller_or_equals_than() {
        let input = Expr::Binary {
            operator: "<=".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 5.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());

        let input = Expr::Binary {
            operator: "<=".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());
    }

    #[test]
    fn it_assert_numeric_inputs() {
        let input = Expr::Binary {
            operator: "+".into(),
            left: Box::new(Expr::Num { value: 1.0 }),
            right: Box::new(Expr::Str {
                value: "hello".into(),
            }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_err());
        assert_eq!(
            String::from("expected two numbers, got number + string"),
            result.unwrap_err()
        );
    }

    #[test]
    fn it_evaluates_a_logical_and_operator() {
        let input = Expr::Binary {
            operator: "&&".into(),
            left: Box::new(Expr::Num { value: 1.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(2.0), result.unwrap());
    }

    #[test]
    fn it_does_not_evaluate_rhs_if_lhs_is_false_on_a_and_operator() {
        let input = Expr::Binary {
            operator: "&&".into(),
            left: Box::new(Expr::Bool { value: false }),
            right: Box::new(Expr::Error),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_logical_or_operator() {
        let input = Expr::Binary {
            operator: "||".into(),
            left: Box::new(Expr::Bool { value: false }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(2.0), result.unwrap());
    }

    #[test]
    fn it_does_not_evaluate_rhs_if_lhs_is_truthy_on_a_or_operator() {
        let input = Expr::Binary {
            operator: "||".into(),
            left: Box::new(Expr::Str { value: "".into() }),
            right: Box::new(Expr::Error),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Str("".into()), result.unwrap());
    }

    #[test]
    fn it_does_not_evaluate_invalid_binary_operators() {
        let input = Expr::Binary {
            operator: "?".into(),
            left: Box::new(Expr::Num { value: 1.0 }),
            right: Box::new(Expr::Num { value: 1.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_err());
        assert_eq!(
            String::from("cannot apply binary operator ?"),
            result.unwrap_err()
        );
    }

    #[test]
    fn it_evaluates_an_equality_operator() {
        let input = Expr::Binary {
            operator: "==".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());

        let input = Expr::Binary {
            operator: "==".into(),
            left: Box::new(Expr::Bool { value: false }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
    }

    #[test]
    fn it_evaluates_an_inequality_operator() {
        let input = Expr::Binary {
            operator: "!=".into(),
            left: Box::new(Expr::Bool { value: false }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(true), result.unwrap());

        let input = Expr::Binary {
            operator: "!=".into(),
            left: Box::new(Expr::Num { value: 2.0 }),
            right: Box::new(Expr::Num { value: 2.0 }),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
    }

    #[test]
    fn it_evaluates_an_if_else_expr() {
        let input = Expr::If {
            cond: Box::new(Expr::Num { value: 0.0 }),
            then: Box::new(Expr::Num { value: 1.0 }),
            otherwise: Some(Box::new(Expr::Num { value: 2.0 })),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(1.0), result.unwrap());

        let input = Expr::If {
            cond: Box::new(Expr::Bool { value: false }),
            then: Box::new(Expr::Num { value: 1.0 }),
            otherwise: Some(Box::new(Expr::Num { value: 2.0 })),
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(2.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_an_if_expr_without_else() {
        let input = Expr::If {
            cond: Box::new(Expr::Bool { value: false }),
            then: Box::new(Expr::Num { value: 1.0 }),
            otherwise: None,
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
    }

    #[test]
    fn it_evaluates_a_block() {
        let input = Expr::Block {
            exprs: vec![Expr::Num { value: 1.0 }, Expr::Num { value: 2.0 }],
        };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Num(2.0), result.unwrap());
    }

    #[test]
    fn it_evaluates_an_empty_block_to_false() {
        let input = Expr::Block { exprs: vec![] };

        let result = evaluate(input, &mut Env::new());

        assert!(result.is_ok());
        assert_eq!(LValue::Bool(false), result.unwrap());
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
