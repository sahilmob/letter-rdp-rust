use super::nodes::*;
use super::tokenizer::Tokenizer;
use std::fmt::Debug;
use std::str::FromStr;

pub struct Parser {
    string: String,
    lookahead: Option<Token>,
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn parse<T: FromStr>(&mut self, string: String) -> Program<T>
    where
        <T as FromStr>::Err: Debug,
    {
        self.string = string.clone();
        self.tokenizer.init(string);
        self.lookahead = self.tokenizer.get_next_token();
        return self.program();
    }

    // Program
    //  : Literal
    //  ;
    fn program<T: FromStr>(&mut self) -> Program<T>
    where
        <T as FromStr>::Err: Debug,
    {
        return Program {
            value: self.literal(),
        };
    }

    // Literal
    // : NumericLiteral
    // | StringLiteral
    // :
    fn literal<T: FromStr>(&mut self) -> Literal<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let token = &self.lookahead;

        match token {
            Some(t) => {
                if t.typ == "NUMBER" {
                    return self.numeric_literal::<T>();
                } else if t.typ == "STRING" {
                    return self.string_literal::<T>();
                } else {
                    panic!("Unsupported token type {}", t.typ);
                }
            }
            None => {
                panic!("Unexpected end of file");
            }
        }
    }

    // NumericLiteral
    //  : STRING
    //  ;
    fn string_literal<T: FromStr>(&mut self) -> Literal<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let token: Token = self.eat("STRING");

        return Literal {
            typ: String::from("StringLiteral"),
            value: token.value.parse::<T>().unwrap(),
        };
    }

    // NumericLiteral
    //  : NUMBER
    //  ;
    fn numeric_literal<T: FromStr>(&mut self) -> Literal<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let token: Token = self.eat("NUMBER");

        return Literal {
            typ: String::from("NumericLiteral"),
            value: token.value.parse::<T>().unwrap(),
        };
    }

    fn eat(&mut self, token_type: &str) -> Token {
        let token: Option<Token> = self.lookahead.clone();
        match token {
            None => panic!("Unexpected end of input, expected: {}", token_type),
            Some(t) => {
                if t.typ != token_type {
                    panic!("Unexpected token: {}, expected: {}", t.typ, token_type)
                }

                self.lookahead = self.tokenizer.get_next_token();

                return t;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prints_number() {
        let mut parser = Parser {
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

        let result = parser.parse::<i64>(String::from("  1"));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("NumericLiteral"),
                    value: 1
                }
            }
        )
    }

    #[test]
    fn prints_double_quotes_string() {
        let mut parser = Parser {
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

        let result = parser.parse::<String>(String::from(r#""a""#));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }

    #[test]
    fn prints_single_quotes_string() {
        let mut parser = Parser {
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

        let result = parser.parse::<String>(String::from(r#"   'a'"#));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }

    #[test]
    fn ignores_single_line_comments() {
        let mut parser = Parser {
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

        let result = parser.parse::<String>(String::from(
            r#"   
            // comment
        'a'
        "#,
        ));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }

    #[test]
    fn ignores_multiline_comments() {
        let mut parser = Parser {
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

        let result = parser.parse::<String>(String::from(
            r#"   
            /*
            *  a comment
            */
        'a'
        "#,
        ));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }
}
