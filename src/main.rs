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

fn main() {}

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

    fn it_ignores_comments() {
        let input = "# hi i am a comment\n1;";

        let result = parse_string(input);

        assert_vec_eq(&[Expression::Num { value: 1.0 }], &result);
    }
}
