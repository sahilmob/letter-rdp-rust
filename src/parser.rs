use super::nodes::*;
use super::tokenizer::Tokenizer;

pub struct Parser {
    pub string: String,
    pub lookahead: Option<Token>,
    pub tokenizer: Tokenizer,
}

impl Parser {
    pub fn parse(&mut self, string: String) -> Program {
        self.string = string.clone();
        self.tokenizer.init(string);
        self.lookahead = self.tokenizer.get_next_token();
        return self.program();
    }

    // Program
    //  : StatementList
    //  ;
    fn program(&mut self) -> Program {
        return Program {
            typ: String::from("Program"),
            body: self.statement_list(),
        };
    }

    // StatementList
    // : Statement
    // | StatementList Statement
    // ;
    fn statement_list(&mut self) -> Vec<ExpressionStatement> {
        let mut statement_list = Vec::new();
        statement_list.push(self.statement());

        while !self.lookahead.is_none() {
            statement_list.push(self.statement())
        }

        return statement_list;
    }

    // Statement
    // : ExpressionStatement
    // ;
    fn statement(&mut self) -> ExpressionStatement {
        return self.expression_statement();
    }

    // ExpressionStatement
    // : Expression ';'
    // ;
    fn expression_statement(&mut self) -> ExpressionStatement {
        let expression = self.expression();
        self.eat(";");
        return ExpressionStatement {
            typ: String::from("ExpressionStatement"),
            expression,
        };
    }

    // Expression
    // : Literal
    // ;
    fn expression(&mut self) -> Literal {
        return self.literal();
    }

    // Literal
    // : NumericLiteral
    // | StringLiteral
    // :
    fn literal(&mut self) -> Literal {
        let token = &self.lookahead;

        match token {
            Some(t) => {
                if t.typ == "NUMBER" {
                    return self.numeric_literal();
                } else if t.typ == "STRING" {
                    return self.string_literal();
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
    fn string_literal(&mut self) -> Literal {
        let token: Token = self.eat("STRING");

        return Literal::StringLiteral {
            typ: String::from("StringLiteral"),
            value: token.value.parse::<String>().unwrap(),
        };
    }

    // NumericLiteral
    //  : NUMBER
    //  ;
    fn numeric_literal(&mut self) -> Literal {
        let token: Token = self.eat("NUMBER");

        return Literal::NumericLiteral {
            typ: String::from("NumericLiteral"),
            value: token.value.parse::<i64>().unwrap(),
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
