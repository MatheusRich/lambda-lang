mod input_stream;
mod token;
mod token_stream;
mod parser;
mod expression;
use input_stream::InputStream;
use token::Token;
use expression::Expression;
use token_stream::TokenStream;
use parser::Parser;

fn main() {
    let input = String::from("if 123.45");
    let input_stream = InputStream::new(input);
    let token_stream = TokenStream::new(input_stream);
    let mut parser = Parser::new(token_stream);

    println!("PARSED SUCCESSFULLY: {:?}", parser.parse());
}
