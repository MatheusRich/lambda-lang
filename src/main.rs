mod input_stream;
mod token;
mod token_stream;
use input_stream::InputStream;
use token::Token;
use token_stream::TokenStream;

fn main() {
    let input = String::from("# asdf 123\n \t123.45()[]{},;");
    let input_stream = InputStream::new(input);
    let mut token_stream = TokenStream::new(input_stream);

    if let Token::Num { value: s } = token_stream.read_next().unwrap() {
        assert_eq!(123.45f64, s);
    } else {
        panic!("error!")
    }
}
