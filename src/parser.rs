use super::{Expr, Token, TokenStream};
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

    pub fn parse(&mut self) -> Vec<Expr> {
        self.parse_toplevel()
    }

    fn parse_toplevel(&mut self) -> Vec<Expr> {
        let mut expressions = Vec::<Expr>::new();

        while !self.input.is_eof() {
            expressions.push(self.parse_expression());

            self.skip_punc(";");
        }

        expressions
    }

    fn parse_expression(&mut self) -> Expr {
        let left = self.parse_atom();
        let maybe_bin = self.maybe_binary(left, 0);

        self.maybe_call(maybe_bin)
    }

    fn parse_var_name(&mut self) -> Expr {
        match self.input.next() {
            Some(Token::Var { value }) => Expr::Var { name: value },
            Some(other) => {
                self.input
                    .syntax_error(&format!("Expecting variable name, got '{}'", other));

                Expr::Error
            }
            None => {
                self.input
                    .syntax_error("Expecting variable name, but got to end of input");

                Expr::Error
            }
        }
    }

    fn maybe_call(&mut self, expr: Expr) -> Expr {
        if self.is_punc("(") {
            self.parse_call(expr)
        } else {
            expr
        }
    }

    fn parse_call(&mut self, func: Expr) -> Expr {
        Expr::Call {
            func: Box::new(func),
            args: self.delimited("(", ")", ",", "expression"),
        }
    }

    fn maybe_binary(&mut self, left: Expr, my_precedence: usize) -> Expr {
        match self.input.peek() {
            Some(Token::Op { value }) => self.parse_binary(left, value, my_precedence),
            _ => left,
        }
    }

    fn parse_binary(&mut self, left: Expr, op: String, my_precedence: usize) -> Expr {
        let his_precedence = self.precedence[&op];
        if his_precedence > my_precedence {
            self.input.next();
            let left = self.parse_atom();

            let new_left = if op == "=" {
                Expr::Assign {
                    operator: op,
                    left: Box::new(left.clone()),
                    right: Box::new(self.maybe_binary(left, his_precedence)),
                }
            } else {
                Expr::Binary {
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

    fn parse_atom(&mut self) -> Expr {
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

            if self.is_kw("lambda") {
                return self.parse_lambda("lambda");
            }

            if self.is_kw("λ") {
                return self.parse_lambda("λ");
            }

            match self.input.next().expect("Unexpected end of tokens") {
                Token::Num { value } => Expr::Num { value },
                Token::Str { value } => Expr::Str { value },
                Token::Var { value } => Expr::Var { name: value },
                token => {
                    self.unexpected(token);

                    Expr::Error {}
                }
            }
        };

        self.maybe_call(atom)
    }

    fn parse_bool(&mut self) -> Expr {
        let is_true;

        match self.input.next().expect("Should not get here") {
            Token::Kw { value } => is_true = value == "true",
            _ => panic!("Should not get here"),
        };

        Expr::Bool { value: is_true }
    }

    fn parse_if(&mut self) -> Expr {
        self.skip_kw("if");

        let cond = self.parse_expression();

        if !self.is_punc("{") {
            self.skip_kw("then")
        };

        let then = self.parse_expression();

        Expr::If {
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: self.parse_else(),
        }
    }

    fn parse_else(&mut self) -> Option<Box<Expr>> {
        if self.is_kw("else") {
            self.input.next();

            Some(Box::new(self.parse_expression()))
        } else {
            None
        }
    }

    fn parse_prog(&mut self) -> Expr {
        let exprs = self.delimited("{", "}", ";", "expression");

        if exprs.is_empty() {
            return Expr::Bool { value: false };
        }

        if exprs.len() == 1 {
            return exprs.first().unwrap().clone();
        }

        Expr::Block { exprs }
    }

    fn parse_lambda(&mut self, lambda_sign: &str) -> Expr {
        self.skip_kw(lambda_sign);

        Expr::Lambda {
            vars: self.delimited("(", ")", ",", "var_name"),
            body: Box::new(self.parse_expression()),
        }
    }

    fn delimited(&mut self, start: &str, stop: &str, sep: &str, parser: &str) -> Vec<Expr> {
        let mut vec = Vec::<Expr>::new();
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
