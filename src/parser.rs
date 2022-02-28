#[path = "nodes.rs"]
mod nodes;
#[path = "tokenizer.rs"]
mod tokenizer;
use nodes::*;
use tokenizer::Tokenizer;

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn parse(&self, string: String) -> NumericLiteral {
        return self.program();
    }

    // Program
    //  : NumericLiteral
    //  ;
    fn program(&self) -> NumericLiteral {
        return self.numeric_literal();
    }

    // numeric_literal
    //  : NUMBER
    //  ;
    fn numeric_literal(&self) -> NumericLiteral {
        return NumericLiteral { value: 1 };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prints_number() {
        let parser = Parser {
            tokenizer: Tokenizer {},
        };
        let result = parser.parse(String::from("hello"));

        assert_eq!(result, NumericLiteral { value: 1 })
    }
}
