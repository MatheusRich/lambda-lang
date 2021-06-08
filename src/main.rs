mod expr;
mod input_stream;
mod parser;
mod token;
mod token_stream;
use expr::Expr;
use input_stream::InputStream;
use parser::Parser;
use token::Token;
use token_stream::TokenStream;

fn main() {
    use std::io::{stdin, stdout, Write};

    print!("> ");
    let _ = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid string");
    let mut parser = Parser::new(TokenStream::new(InputStream::new(input)));

    println!("Parsed successfully: {:?}", parser.parse());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_string(input: &str) -> Vec<Expr> {
        let mut parser = Parser::new(TokenStream::new(InputStream::new(String::from(input))));

        parser.parse()
    }

    fn assert_vec_eq(va: &[Expr], vb: &[Expr]) {
        assert_eq!(va.len(), vb.len(), "Vectors have different lengths");

        for (a, b) in va.iter().zip(vb) {
            assert_eq!(a, b);
        }
    }

    fn literal(kind: &str, value: &str) -> Box<Expr> {
        match kind {
            "num" => Box::new(Expr::Num {
                value: value.parse().expect("Invalid float"),
            }),
            _ => panic!("Don't know how to create literal {}", kind),
        }
    }

    #[test]
    fn it_parses_nothing() {
        assert!(parse_string("").is_empty());
    }

    #[test]
    fn it_parses_a_number() {
        let input = "123.45;";

        let result = parse_string(input);

        assert_vec_eq(&[Expr::Num { value: 123.45 }], &result);
    }

    #[test]
    fn it_parses_a_group() {
        let input = "(((123.45)));";

        let result = parse_string(input);

        assert_vec_eq(&[Expr::Num { value: 123.45 }], &result);
    }

    #[test]
    fn it_parses_booleans() {
        let input = "true;false;";

        let result = parse_string(input);

        assert_vec_eq(
            &[
                Expr::Bool { value: true },
                Expr::Bool { value: false },
            ],
            &result,
        );
    }

    #[test]
    fn it_parses_variables() {
        let input = "a_variable;another-variable;";

        let result = parse_string(input);

        assert_vec_eq(
            &[
                Expr::Var {
                    name: String::from("a_variable"),
                },
                Expr::Var {
                    name: String::from("another-variable"),
                },
            ],
            &result,
        );
    }

    #[test]
    fn it_parses_strings() {
        let input = "\"a string\";
                    \"other \\\" string\";";

        let result = parse_string(input);

        assert_vec_eq(
            &[
                Expr::Str {
                    value: String::from("a string"),
                },
                Expr::Str {
                    value: String::from("other \" string"),
                },
            ],
            &result,
        );
    }

    #[test]
    fn it_parses_if_with_then() {
        let input = "if 0 then 1;";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::If {
                cond: literal("num", "0"),
                then: literal("num", "1.0"),
                otherwise: None,
            }],
            &result,
        );
    }

    #[test]
    fn it_parses_if_then_else() {
        let input = "if 0 then 1 else 2;";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::If {
                cond: literal("num", "0"),
                then: literal("num", "1.0"),
                otherwise: Some(literal("num", "2.0")),
            }],
            &result,
        );
    }

    #[test]
    fn it_parses_if_else_with_brackets() {
        let input = "
            if 0 {
                1;
            } else {
                2;
            };
        ";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::If {
                cond: literal("num", "0"),
                then: literal("num", "1.0"),
                otherwise: Some(literal("num", "2.0")),
            }],
            &result,
        );
    }

    #[test]
    fn it_parses_empty_block() {
        let input = "{};";

        let result = parse_string(input);

        assert_vec_eq(&[Expr::Bool { value: false }], &result);
    }

    #[test]
    fn it_unwraps_block_with_just_one_expression() {
        let input = "{1;};";

        let result = parse_string(input);

        assert_vec_eq(&[Expr::Num { value: 1.0 }], &result);
    }

    #[test]
    fn it_parses_multi_expression_block() {
        let input = "
            {
                1;
                a_var;
            };
        ";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::Block {
                exprs: vec![
                    Expr::Num { value: 1.0 },
                    Expr::Var {
                        name: String::from("a_var"),
                    },
                ],
            }],
            &result,
        );
    }

    #[test]
    fn it_parses_simple_lambdas() {
        let input = "
            lambda () 1;
            λ () 2;
        ";

        let result = parse_string(input);

        assert_vec_eq(
            &[
                Expr::Lambda {
                    vars: vec![],
                    body: literal("num", "1"),
                },
                Expr::Lambda {
                    vars: vec![],
                    body: literal("num", "2"),
                },
            ],
            &result,
        );
    }

    #[test]
    fn it_parses_a_lambda_with_args() {
        let input = "
            lambda (a_var, other-var,) {
                1
            };
        ";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::Lambda {
                vars: vec![
                    Expr::Var {
                        name: String::from("a_var"),
                    },
                    Expr::Var {
                        name: String::from("other-var"),
                    },
                ],
                body: literal("num", "1"),
            }],
            &result,
        );
    }

    #[test]
    #[should_panic(expected = "Expecting variable name, got '1'")]
    fn it_only_allows_variable_names_in_lambda_variable_section() {
        let input = "
            lambda (a_var, 1) {
                1
            };
        ";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::Lambda {
                vars: vec![
                    Expr::Var {
                        name: String::from("a_var"),
                    },
                    Expr::Var {
                        name: String::from("other-var"),
                    },
                ],
                body: literal("num", "1"),
            }],
            &result,
        );
    }

    #[test]
    #[should_panic(expected = "Expecting variable name, but got to end of input")]
    fn it_fails_if_got_to_end_of_input_when_reading_variable_names() {
        let input = "lambda (a_var,";

        let result = parse_string(input);

        assert_vec_eq(
            &[Expr::Lambda {
                vars: vec![
                    Expr::Var {
                        name: String::from("a_var"),
                    },
                    Expr::Var {
                        name: String::from("other-var"),
                    },
                ],
                body: literal("num", "1"),
            }],
            &result,
        );
    }

    #[test]
    fn it_ignores_comments() {
        let input = "# hi i am a comment\n1;";

        let result = parse_string(input);

        assert_vec_eq(&[Expr::Num { value: 1.0 }], &result);
    }
}
