use super::nodes::*;
use super::tokenizer::Tokenizer;
use std::fmt::Debug;
use std::str::FromStr;

pub struct Parser {
    pub string: String,
    pub lookahead: Option<Token>,
    pub tokenizer: Tokenizer,
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
