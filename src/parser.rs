#[path = "nodes.rs"]
mod nodes;
#[path = "tokenizer.rs"]
mod tokenizer;
use nodes::*;
use tokenizer::Tokenizer;

pub struct Parser {
    string: String,
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn parse(&self) -> Program {
        return self.program();
    }

    // Program
    //  : NumericLiteral
    //  ;
    fn program(&self) -> Program {
        return Program {
            value: self.numeric_literal(),
        };
    }

    // numeric_literal
    //  : NUMBER
    //  ;
    fn numeric_literal(&self) -> NumericLiteral {
        return NumericLiteral {
            value: match self.string.parse::<i64>() {
                ok => ok.unwrap(),
            },
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prints_number() {
        let parser = Parser {
            string: String::from("1"),
            tokenizer: Tokenizer {},
        };
        let result = parser.parse();

        assert_eq!(
            result,
            Program {
                value: NumericLiteral { value: 1 }
            }
        )
    }
}
