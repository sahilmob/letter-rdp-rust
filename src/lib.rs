mod nodes;
mod parser;
mod tokenizer;
use self::parser::Parser;
use self::tokenizer::Tokenizer;
pub use nodes::*;

pub fn init<'a>() -> Parser<'a> {
    let parser = Parser {
        string: String::from(""),
        lookahead: Some(Token {
            typ: "",
            value: String::from(""),
        }),
        tokenizer: Tokenizer {
            string: String::from(""),
            cursor: 0,
        },
    };

    return parser;
}
