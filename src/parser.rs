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
    // | VariableStatement
    // ;
    fn statement(&mut self) -> Statement<'a> {
        match self.lookahead.as_ref().unwrap().typ {
            ";" => self.empty_statement(),
            "{" => self.block_statement(),
            "let" => self.variable_statement(),
            _ => self.expression_statement(),
        }
    }

    // VariableStatement
    //     : "let" VariableDeclarationList ";"
    //     ;
    fn variable_statement(&mut self) -> Statement<'a> {
        self.eat("let");
        let declarations = self.variable_declarations_list();
        self.eat(";");

        return Statement::VariableStatement(VariableStatement {
            typ: "VariableStatement",
            declarations,
        });
    }

    // VariableDeclarationList
    //  : VariableDeclaration
    //  | VariableDeclarationList VariableDeclaration
    //  ;
    fn variable_declarations_list(&mut self) -> Vec<VariableDeclaration<'a>> {
        let mut declarations = Vec::new();

        loop {
            declarations.push(self.variable_declaration());
            if self.lookahead.clone().unwrap().typ != "," {
                break;
            }
            self.eat(",");
        }

        return declarations;
    }

    // VariableDeclaration
    //  : Identifier OptVariableInitializer
    //  ;

    fn variable_declaration(&mut self) -> VariableDeclaration<'a> {
        let id = self.identifier();
        let lookahead_type = self.lookahead.clone().unwrap().typ;

        let init = match lookahead_type {
            ";" => None,
            "," => None,
            _ => Some(self.variable_initializer()),
        };
        return VariableDeclaration {
            typ: "VariableDeclaration",
            id,
            init,
        };
    }

    // VariableInitializer
    //  : SIMPLE_ASSIGN AssignmentExpression
    //  ;
    fn variable_initializer(&mut self) -> VariableInitializer<'a> {
        self.eat("SIMPLE_ASSIGN");
        match self.assignment_expression() {
            Expression::Literal(id) => VariableInitializer::Literal(id),
            Expression::AssignmentExpression(ae) => VariableInitializer::AssignmentExpression(ae),
            _ => panic!("Unexpected expression, expected assignment expression."),
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
    // : AssignmentExpression
    // ;
    fn expression(&mut self) -> Expression<'a> {
        return self.assignment_expression();
    }

    // AssignmentExpression
    // : AdditiveExpression
    // | LeftHandSideExpression AssignmentOperator AssignmentExpression
    // ;
    fn assignment_expression(&mut self) -> Expression<'a> {
        let left = self.additive_expression();

        if !self.is_assignment_operator(self.lookahead.as_ref().unwrap().typ) {
            return left;
        }

        return Expression::AssignmentExpression(AssignmentExpression {
            typ: "AssignmentExpression",
            operator: self.assignment_operator().value,
            left: self.check_valid_assignment_target(left),
            right: Box::new(self.assignment_expression()),
        });
    }
    fn check_valid_assignment_target(&self, node: Expression<'a>) -> Identifier<'a> {
        match node {
            Expression::LeftHandSideExpression(lhse) => match lhse {
                LeftHandSideExpression::Identifier(i) => Identifier {
                    typ: "Identifier",
                    name: i.name,
                },
            },
            _ => panic!("Invalid left-hand side in assignment expression"),
        }
    }

    // LeftHandSideExpression
    // : Identifier
    // ;
    fn left_hand_side_expression(&mut self) -> Expression<'a> {
        return Expression::LeftHandSideExpression(LeftHandSideExpression::Identifier(
            self.identifier(),
        ));
    }

    // Identifier
    // : IDENTIFIER
    // ;
    fn identifier(&mut self) -> Identifier<'a> {
        let name = self.eat("IDENTIFIER").clone().value;
        return Identifier {
            typ: "Identifier",
            name: name,
        };
    }

    fn is_assignment_operator(&self, token_type: &str) -> bool {
        return token_type == "SIMPLE_ASSIGN" || token_type == "COMPLEX_ASSIGN";
    }

    // AssignmentOperator
    // : SIMPLE_ASSIGN
    // | COMPLEX_ASSIGN
    // ;
    fn assignment_operator(&mut self) -> Token {
        if self.lookahead.as_ref().unwrap().typ == "SIMPLE_ASSIGN" {
            return self.eat("SIMPLE_ASSIGN");
        }
        return self.eat("COMPLEX_ASSIGN");
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

    //  PrimaryExpression
    // : Literal
    // ; ParenthesizedExpression
    // ; LeftHandSideExpression
    // ;
    fn primary_expression(&mut self) -> Expression<'a> {
        let token = &self.lookahead;
        if self.is_literal(self.lookahead.as_ref().unwrap().typ) {
            return self.literal();
        }
        match token {
            Some(t) => {
                if t.typ == "(" {
                    return self.parenthesized_expression();
                } else {
                    return self.left_hand_side_expression();
                }
            }
            None => panic!("Unexpected primary expression"),
        }
    }

    fn is_literal(&self, token_type: &str) -> bool {
        return token_type == "NUMBER" || token_type == "STRING";
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
