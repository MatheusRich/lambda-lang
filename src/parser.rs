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

            self.skip_punc(";");
        }

        expressions
    }

    fn parse_expression(&mut self) -> Expression {
        let left = self.parse_atom();
        let maybe_bin = self.maybe_binary(left, 0);

        self.maybe_call(maybe_bin)
    }

    fn parse_var_name(&mut self) -> Expression {
        match self.input.next() {
            Some(Token::Var { value }) => Expression::Var { name: value },
            Some(other) => {
                self.input
                    .syntax_error(&format!("Expecting variable name, got '{}'", other));

                Expression::Error
            }
            None => {
                self.input
                    .syntax_error("Expecting variable name, but got to end of input");

                Expression::Error
            }
        }
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

            if self.is_punc("{") {
                return self.parse_prog();
            }

            if self.is_kw("if") {
                return self.parse_if();
            };

            if self.is_kw("true") || self.is_kw("false") {
                return self.parse_bool();
            }

            if self.is_kw("lambda")
            /*|| is_kw("λ")*/
            {
                return self.parse_lambda("lambda");
            }

            match self.input.next().expect("Unexpected end of tokens") {
                Token::Num { value } => Expression::Num { value },
                Token::Str { value } => Expression::String { value },
                Token::Var { value } => Expression::Var { name: value },
                token => {
                    self.unexpected(token);

                    Expression::Error {}
                }
            }
        };

        self.maybe_call(atom)
    }

    fn parse_bool(&mut self) -> Expression {
        let is_true;

        match self.input.next().expect("Should not get here") {
            Token::Kw { value } => is_true = value == "true",
            _ => panic!("Should not get here"),
        };

        Expression::Bool { value: is_true }
    }

    fn parse_if(&mut self) -> Expression {
        self.skip_kw("if");

        let cond = self.parse_expression();

        if !self.is_punc("{") {
            self.skip_kw("then")
        };

        let then = self.parse_expression();

        Expression::If {
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: self.parse_else(),
        }
    }

    fn parse_else(&mut self) -> Option<Box<Expression>> {
        if self.is_kw("else") {
            self.input.next();

            Some(Box::new(self.parse_expression()))
        } else {
            None
        }
    }

    fn parse_prog(&mut self) -> Expression {
        let prog = self.delimited("{", "}", ";", "expression");

        if prog.is_empty() {
            return Expression::Bool { value: false };
        }

        if prog.len() == 1 {
            return prog.first().unwrap().clone();
        }

        Expression::Prog { prog }
    }

    fn parse_lambda(&mut self, lambda_sign: &str) -> Expression {
        self.skip_kw(lambda_sign);

        Expression::Lambda {
            vars: self.delimited("(", ")", ",", "var_name"),
            body: Box::new(self.parse_expression()),
        }
    }

    fn delimited(&mut self, start: &str, stop: &str, sep: &str, parser: &str) -> Vec<Expression> {
        let mut vec = Vec::<Expression>::new();
        let mut first = true;
        // let mut i = 1;

        self.skip_punc(start);
        while !self.input.is_eof() {
            // println!("loop #{}" , i);
            // println!("cur token #{:?}" , self.input.peek());
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
                "var_name" => self.parse_var_name(),
                _ => panic!("Unknown parser {}", parser),
            };

            vec.push(expr);
            // i += 1;
        }
        self.skip_punc(stop);

        vec
    }

    fn skip_punc(&mut self, expected: &str) {
        if self.is_punc(expected) {
            self.input.next();
        } else {
            println!("\n\n\ncurrent: {}", self.input.peek().unwrap());
            println!("expected: {}\n\n\n", expected);
            self.input
                .syntax_error(&format!("Expected punctuation {}", expected));
        };
    }

    fn skip_kw(&mut self, expected: &str) {
        if self.is_kw(expected) {
            self.input.next()
        } else {
            self.input
                .syntax_error(&format!("Expected keyword {}", expected));

            None
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

    fn is_kw(&mut self, expected: &str) -> bool {
        match self.input.peek() {
            Some(Token::Kw { value }) => value == expected,
            _ => false,
        }
    }

    fn unexpected(&mut self, token: Token) {
        self.input
            .syntax_error(&format!("Unexpected token '{}'", token));
    }
}
