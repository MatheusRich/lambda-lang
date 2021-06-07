mod expression;
mod input_stream;
mod parser;
mod token;
mod token_stream;
use expression::Expression;
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

    fn parse_string(input: &str) -> Vec<Expression> {
        let mut parser = Parser::new(TokenStream::new(InputStream::new(String::from(input))));

        parser.parse()
    }

    fn assert_vec_eq(va: &[Expression], vb: &[Expression]) {
        assert!((va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| a == b))
    }

    fn literal(kind: &str, value: &str) -> Box<Expression> {
        match kind {
            "num" => Box::new(Expression::Num {
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

        assert_vec_eq(&[Expression::Num { value: 123.45 }], &result);
    }

    #[test]

    fn it_parses_a_group() {
        let input = "(((123.45)));";

        let result = parse_string(input);

        assert_vec_eq(&[Expression::Num { value: 123.45 }], &result);
    }

    #[test]

    fn it_parses_booleans() {
        let input = "true;false;";

        let result = parse_string(input);

        assert_vec_eq(
            &[
                Expression::Bool { value: true },
                Expression::Bool { value: false },
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
                Expression::Var {
                    name: String::from("a_variable"),
                },
                Expression::Var {
                    name: String::from("another-variable"),
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
            &[Expression::If {
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
            &[Expression::If {
                cond: literal("num", "0"),
                then: literal("num", "1.0"),
                otherwise: Some(literal("num", "2.0")),
            }],
            &result,
        );
    }

    #[test]

    fn it_ignores_comments() {
        let input = "# hi i am a comment\n1;";

        let result = parse_string(input);

        assert_vec_eq(&[Expression::Num { value: 1.0 }], &result);
    }
}
