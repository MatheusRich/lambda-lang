use super::{Expression, Token, TokenStream};
use std::collections::HashMap;

pub struct Parser {
    input: TokenStream,
    precedence: HashMap<String, usize>,
}

impl Parser {
    pub fn new(input: TokenStream) -> Parser {
        Parser {
            input,
            precedence: [
                (String::from("="), 1),
                (String::from("||"), 2),
                (String::from("&&"), 3),
                (String::from("<"), 7),
                (String::from(">"), 7),
                (String::from("<="), 7),
                (String::from(">="), 7),
                (String::from("=="), 7),
                (String::from("!="), 7),
                (String::from("+"), 10),
                (String::from("-"), 10),
                (String::from("*"), 20),
                (String::from("/"), 20),
                (String::from("%"), 20),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        self.parse_toplevel()
    }

    fn parse_toplevel(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::<Expression>::new();

        while !self.input.is_eof() {
            expressions.push(self.parse_expression());

            if !self.input.is_eof() {
                self.skip_punc(";");
            }
        }

        expressions
    }

    fn parse_expression(&mut self) -> Expression {
        let left = self.parse_atom();
        let maybe_bin = self.maybe_binary(left, 0);

        self.maybe_call(maybe_bin)
    }

    fn maybe_call(&mut self, expr: Expression) -> Expression {
        if self.is_punc("(") {
            self.parse_call(expr)
        } else {
            expr
        }
    }

    fn parse_call(&mut self, func: Expression) -> Expression {
        Expression::Call {
            func: Box::new(func),
            args: self.delimited("(", ")", ",", "expression"),
        }
    }

    fn maybe_binary(&mut self, left: Expression, my_precedence: usize) -> Expression {
        match self.input.peek() {
            Some(Token::Op { value }) => self.parse_binary(left, value, my_precedence),
            _ => left,
        }
    }

    fn parse_binary(&mut self, left: Expression, op: String, my_precedence: usize) -> Expression {
        let his_precedence = self.precedence[&op];
        if his_precedence > my_precedence {
            self.input.next();
            let left = self.parse_atom();

            let new_left = if op == "=" {
                Expression::Assign {
                    operator: op,
                    left: Box::new(left.clone()),
                    right: Box::new(self.maybe_binary(left, his_precedence)),
                }
            } else {
                Expression::Binary {
                    operator: op,
                    left: Box::new(left.clone()),
                    right: Box::new(self.maybe_binary(left, his_precedence)),
                }
            };

            self.maybe_binary(new_left, my_precedence)
        } else {
            left
        }
    }

    fn parse_atom(&mut self) -> Expression {
        let atom = {
            if self.is_punc("(") {
                self.input.next();
                let exp = self.parse_expression();
                self.skip_punc(")");

                return exp;
            }
            // if (is_punc("{")) return parse_prog();
            // if (is_kw("if")) return parse_if();
            // if (is_kw("true") || is_kw("false")) return parse_bool();
            // if (is_kw("lambda") || is_kw("λ")) {
            //     input.next();
            //     return parse_lambda();
            // }

            // if (tok.type == "var" || tok.type == "num" || tok.type == "str") return tok;

            match self.input.peek() {
                Some(Token::Num { value }) => Expression::Num { value },
                _ => {
                    self.unexpected();

                    Expression::Error {}
                }
            }
        };

        self.maybe_call(atom)
    }

    fn delimited(
        &mut self,
        start: &str,
        stop: &str,
        sep: &str,
        parser: &str,
    ) -> Vec<Box<Expression>> {
        let mut vec = Vec::<Box<Expression>>::new();
        let mut first = false;

        self.skip_punc(start);
        while !self.input.is_eof() {
            if self.is_punc(stop) {
                break;
            }
            if first {
                first = false
            } else {
                self.skip_punc(sep)
            }
            if self.is_punc(stop) {
                break;
            }

            let expr = match parser {
                "expression" => self.parse_expression(),
                _ => panic!("Unknown parser {}", parser)
            };

            vec.push(Box::new(expr));
        }
        self.skip_punc(stop);

        vec
    }

    fn skip_punc(&mut self, expected: &str) {
        if self.is_punc(expected) {
            self.input.next()
        } else {
            self.input.syntax_error(&format!("Expected {} here", expected));

            None // Some(Token::Error)
        };
    }

    fn is_punc(&mut self, expected: &str) -> bool {
        match self.input.peek() {
            Some(Token::Punc { value }) => value == expected,
            _ => false,
        }
    }

    fn is_op(&mut self, expected: &str) -> bool {
        match self.input.peek() {
            Some(Token::Op { value }) => expected == "any" || value == expected,
            _ => false,
        }
    }

    fn unexpected(&mut self) {
        let token = self.input.peek().unwrap();
        self.input.syntax_error(&format!("Unexpected token '{}'", token));
    }
}
