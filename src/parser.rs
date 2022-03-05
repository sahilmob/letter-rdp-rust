use super::nodes::*;
use super::tokenizer::Tokenizer;

#[derive(Default, Debug)]
pub struct Parser<'a> {
    pub string: &'a str,
    pub lookahead: Option<Token<'a>>,
    pub tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, string: &'a str) -> Program {
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
            typ: "Program",
            body: self.statement_list(),
        };
    }

    // StatementList
    // : Statement
    // | StatementList Statement
    // ;
    fn statement_list(&mut self) -> Vec<ExpressionStatement<'a>> {
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
    fn statement(&mut self) -> ExpressionStatement<'a> {
        return self.expression_statement();
    }

    // ExpressionStatement
    // : Expression ';'
    // ;
    fn expression_statement(&mut self) -> ExpressionStatement<'a> {
        let expression = self.expression();
        self.eat(";");
        return ExpressionStatement {
            typ: "ExpressionStatement",
            expression,
        };
    }

    // Expression
    // : Literal
    // ;
    fn expression(&mut self) -> Literal<'a> {
        return self.literal();
    }

    // Literal
    // : NumericLiteral
    // | StringLiteral
    // :
    fn literal(&mut self) -> Literal<'a> {
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
    fn string_literal(&mut self) -> Literal<'a> {
        let token: Token = self.eat("STRING");
        let value = token.value;
        return Literal::StringLiteral {
            typ: "StringLiteral",
            value: value[1..value.len() - 1].to_string(),
        };
    }

    // NumericLiteral
    //  : NUMBER
    //  ;
    fn numeric_literal(&mut self) -> Literal<'a> {
        let token: Token = self.eat("NUMBER");

        return Literal::NumericLiteral {
            typ: "NumericLiteral",
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
