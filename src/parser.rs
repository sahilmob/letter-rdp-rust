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
        self.lookahead = self.tokenizer.next();
        return self.program();
    }

    // Program
    //  : StatementList
    //  ;
    fn program(&mut self) -> Program {
        return Program {
            typ: "Program",
            body: self.statement_list(""),
        };
    }

    // StatementList
    // : Statement
    // | StatementList Statement
    // ;
    fn statement_list(&mut self, stop_lookahead: &str) -> Vec<Statement<'a>> {
        let mut statement_list = Vec::new();
        statement_list.push(self.statement());

        while !self.lookahead.is_none() && self.lookahead.as_ref().unwrap().typ != stop_lookahead {
            statement_list.push(self.statement())
        }

        return statement_list;
    }

    // Statement
    // : ExpressionStatement
    // | BlockStatement
    // | EmptyStatement
    // ;
    fn statement(&mut self) -> Statement<'a> {
        match self.lookahead.as_ref().unwrap().typ {
            ";" => self.empty_statement(),
            "{" => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    // EmptyStatement
    // : ";"
    // ;
    fn empty_statement(&mut self) -> Statement<'a> {
        self.eat(";");
        return Statement::EmptyStatement {
            typ: "EmptyStatement",
        };
    }

    // BlockStatement
    // : "{" OptStatementList "}"
    // ;
    fn block_statement(&mut self) -> Statement<'a> {
        self.eat("{");
        let body = if self.lookahead.as_ref().unwrap().typ != "}" {
            self.statement_list("}")
        } else {
            vec![]
        };
        self.eat("}");

        return Statement::BlockStatement(BlockStatement {
            typ: "BlockStatement",
            body,
        });
    }

    // ExpressionStatement
    // : Expression ";"
    // ;
    fn expression_statement(&mut self) -> Statement<'a> {
        let expression = self.expression();
        self.eat(";");
        return Statement::ExpressionStatement(ExpressionStatement {
            typ: "ExpressionStatement",
            expression,
        });
    }

    // Expression
    // : Literal
    // ;
    fn expression(&mut self) -> Expression<'a> {
        return self.additive_expression();
    }

    // AdditiveExpression
    // : MultiplicativeExpression
    // | AdditiveExpression ADDITIVE_OPERATOR Literal
    // ;
    fn additive_expression(&mut self) -> Expression<'a> {
        let mut left = self.multiplicative_expression();

        while self.lookahead.as_ref().unwrap().typ == "ADDITIVE_OPERATOR" {
            let operator = self.eat("ADDITIVE_OPERATOR").value;
            let right = self.multiplicative_expression();

            left = Expression::BinaryExpression(BinaryExpression {
                typ: "BinaryExpression",
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        return left;
    }

    // MultiplicativeExpression
    // : PrimaryExpression
    // | MultiplicativeExpression MULTIPLICATIVE_OPERATOR PrimaryExpression
    // ;
    fn multiplicative_expression(&mut self) -> Expression<'a> {
        let mut left = self.primary_expression();

        while self.lookahead.as_ref().unwrap().typ == "MULTIPLICATIVE_OPERATOR" {
            let operator = self.eat("MULTIPLICATIVE_OPERATOR").value;
            let right = self.primary_expression();

            left = Expression::BinaryExpression(BinaryExpression {
                typ: "BinaryExpression",
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        return left;
    }

    // // Generic binary expression
    // fn _binary_expression(&self, builder_name: &str, operator_token: &str) {
    //     let mut left = self.[builder_name]();

    //     while self.lookahead.as_ref().unwrap().typ == "MULTIPLICATIVE_OPERATOR" {
    //         let operator = self.eat("MULTIPLICATIVE_OPERATOR").value;
    //         let right = self.primary_expression();

    //         left = Expression::BinaryExpression(BinaryExpression {
    //             typ: "BinaryExpression",
    //             operator: operator,
    //             left: Box::new(left),
    //             right: Box::new(right),
    //         });
    //     }

    //     return left;
    // }

    //  PrimaryExpression
    // : Literal
    // ;
    fn primary_expression(&mut self) -> Expression<'a> {
        let token = &self.lookahead;
        match token {
            Some(t) => {
                if t.typ == "(" {
                    return self.parenthesized_expression();
                } else {
                    return self.literal();
                }
            }
            None => panic!("Unexpected primary expression"),
        }
    }

    // ParenthesizedExpression
    // "(" Expression ")"
    // ;
    fn parenthesized_expression(&mut self) -> Expression<'a> {
        self.eat("(");
        let expression = self.expression();
        self.eat(")");

        return expression;
    }

    // Literal
    // : NumericLiteral
    // | StringLiteral
    // :
    fn literal(&mut self) -> Expression<'a> {
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
    fn string_literal(&mut self) -> Expression<'a> {
        let token: Token = self.eat("STRING");
        let value = token.value;
        return Expression::Literal(Literal::StringLiteral(StringLiteral {
            typ: "StringLiteral",
            value: value[1..value.len() - 1].to_string(),
        }));
    }

    // NumericLiteral
    //  : NUMBER
    //  ;
    fn numeric_literal(&mut self) -> Expression<'a> {
        let token: Token = self.eat("NUMBER");

        return Expression::Literal(Literal::NumericLiteral(NumericLiteral {
            typ: "NumericLiteral",
            value: token.value.parse::<i64>().unwrap(),
        }));
    }

    fn eat(&mut self, token_type: &str) -> Token<'a> {
        let token: Option<Token> = self.lookahead.clone();
        match token {
            None => panic!("Unexpected end of input, expected: {}", token_type),
            Some(t) => {
                if t.typ != token_type {
                    panic!("Unexpected token: {}, expected: {}", t.typ, token_type)
                }

                self.lookahead = self.tokenizer.next();

                return t;
            }
        }
    }
}
