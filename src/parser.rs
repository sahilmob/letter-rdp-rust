#[path = "tokenizer.rs"]
mod tokenizer;
use tokenizer::Tokenizer;
pub struct Parser {}

impl Parser {
    fn a() {
        let tokenizer = Tokenizer {};
    }
}
