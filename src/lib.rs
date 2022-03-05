mod nodes;
mod parser;
mod tokenizer;
use self::parser::Parser;
use self::tokenizer::Tokenizer;
pub use nodes::*;

pub fn init<'a>() -> Parser<'a> {
    let parser = Parser {
        string: "",
        lookahead: Some(Token {
            typ: "",
            value: String::from(""),
        }),
        tokenizer: Tokenizer {
            string: "",
            cursor: 0,
        },
    };

    return parser;
}
