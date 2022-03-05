mod nodes;
mod parser;
mod tokenizer;
use self::parser::Parser;
pub use nodes::*;

pub fn init<'a>() -> Parser<'a> {
    let parser = Parser::new();
    return parser;
}
