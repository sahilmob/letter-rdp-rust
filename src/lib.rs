mod nodes;
mod parser;
mod tokenizer;
use self::parser::Parser;
use self::tokenizer::Tokenizer;
pub use nodes::*;

pub fn init() -> Parser {
    let parser = Parser {
        string: String::from(""),
        lookahead: Some(Token {
            typ: String::from(""),
            value: String::from(""),
        }),
        tokenizer: Tokenizer {
            string: String::from(""),
            cursor: 0,
        },
    };

    return parser;
}
