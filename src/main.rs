mod input_stream;
mod token;
mod token_stream;
use token::Token;

fn main() {
    let input = String::from(". \t()[]{},;");
    let input_stream = input_stream::InputStream::new(input);
    let mut token_stream = token_stream::TokenStream::new(input_stream);

    if let Token::Punc { value: s } = token_stream.read_next().unwrap() {
        assert_eq!(String::from("("), s);
    } else {
        panic!("error!")
    }
}
