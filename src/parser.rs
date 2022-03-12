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
    // | IfStatement
    // ;
    fn statement(&mut self) -> Statement<'a> {
        match self.lookahead.as_ref().unwrap().typ {
            ";" => self.empty_statement(),
            "{" => self.block_statement(),
            "let" => self.variable_statement(),
            "if" => self.if_statement(),
            _ => self.expression_statement(),
        }
    }

    // IfStatement
    //  : "if" "(" Expression ")" Statement
    //  : "if" "(" Expression ")" Statement "else" Statement
    //  ;

    fn if_statement(&mut self) -> Statement<'a> {
        self.eat("if");
        self.eat("(");
        let test = if self.lookahead.clone().unwrap().typ == "IDENTIFIER" {
            Test::Identifier(self.identifier())
        } else {
            match self.expression() {
                Expression::Literal(lt) => match lt {
                    Literal::StringLiteral(sl) => {
                        Test::Literal(Literal::StringLiteral(StringLiteral { ..sl }))
                    }
                    Literal::NumericLiteral(nl) => {
                        Test::Literal(Literal::NumericLiteral(NumericLiteral { ..nl }))
                    }
                    Literal::BooleanLiteral(bl) => {
                        Test::Literal(Literal::BooleanLiteral(BooleanLiteral { ..bl }))
                    }
                    Literal::NullLiteral(nl) => {
                        Test::Literal(Literal::NullLiteral(NullLiteral { ..nl }))
                    }
                },
                Expression::BinaryExpression(bxp) => {
                    Test::BinaryExpression(BinaryExpression { ..bxp })
                }
                _ => panic!("Unexpected test expression"),
            }
        };

        self.eat(")");
        let consequent = Box::new(self.statement());
        let mut alternate = None;
        if !self.lookahead.is_none() && self.lookahead.clone().unwrap().typ == "else" {
            self.eat("else");
            alternate = Some(Box::new(self.statement()));
        }

        return Statement::IfStatement(IfStatement {
            typ: "IfStatement",
            test,
            consequent,
            alternate,
        });
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
    // : LogicalORExpression
    // | LeftHandSideExpression AssignmentOperator AssignmentExpression
    // ;
    fn assignment_expression(&mut self) -> Expression<'a> {
        let left = self.logical_or_expression();

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

    // EqualityExpression
    //  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
    //  | RelationalExpression
    //  ;
    fn equality_expression(&mut self) -> Expression<'a> {
        let mut left = self.relational_expression();

        while self.lookahead.as_ref().unwrap().typ == "EQUALITY_OPERATOR" {
            let operator = self.eat("EQUALITY_OPERATOR").value;
            let right = self.relational_expression();

            left = Expression::BinaryExpression(BinaryExpression {
                typ: "BinaryExpression",
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        return left;
    }

    // RelationalExpression
    // : AdditiveExpression
    // : AdditiveExpression RELATIONAL_OPERATOR RelationalExpression
    // ;
    fn relational_expression(&mut self) -> Expression<'a> {
        let mut left = self.additive_expression();

        while self.lookahead.as_ref().unwrap().typ == "RELATIONAL_OPERATOR" {
            let operator = self.eat("RELATIONAL_OPERATOR").value;
            let right = self.additive_expression();

            left = Expression::BinaryExpression(BinaryExpression {
                typ: "BinaryExpression",
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        return left;
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
    fn left_hand_side_expression(&mut self) -> LeftHandSideExpression<'a> {
        return LeftHandSideExpression::Identifier(self.identifier());
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

    fn logical_and_expression(&mut self) -> Expression<'a> {
        let mut left = self.equality_expression();

        while self.lookahead.as_ref().unwrap().typ == "LOGICAL_AND" {
            let operator = self.eat("LOGICAL_AND").value;
            let right = self.equality_expression();

            left = Expression::LogicalExpression(LogicalExpression {
                typ: "LogicalExpression",
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        return left;
    }

    fn logical_or_expression(&mut self) -> Expression<'a> {
        let mut left = self.logical_and_expression();

        while self.lookahead.as_ref().unwrap().typ == "LOGICAL_OR" {
            let operator = self.eat("LOGICAL_OR").value;
            let right = self.logical_and_expression();

            left = Expression::LogicalExpression(LogicalExpression {
                typ: "LogicalExpression",
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        return left;
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
            return Expression::Literal(self.literal());
        }
        match token {
            Some(t) => {
                if t.typ == "(" {
                    return self.parenthesized_expression();
                } else {
                    return Expression::LeftHandSideExpression(self.left_hand_side_expression());
                }
            }
            None => panic!("Unexpected primary expression"),
        }
    }

    fn is_literal(&self, token_type: &str) -> bool {
        return token_type == "NUMBER"
            || token_type == "STRING"
            || token_type == "true"
            || token_type == "false"
            || token_type == "null";
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
    // | BooleanLiteral
    // | NullLiteral
    // :
    fn literal(&mut self) -> Literal<'a> {
        let token = &self.lookahead;

        let literal = if let Some(t) = token {
            match t.typ {
                "NUMBER" => Literal::NumericLiteral(self.numeric_literal()),
                "STRING" => Literal::StringLiteral(self.string_literal()),
                "true" => Literal::BooleanLiteral(self.boolean_literal(true)),
                "false" => Literal::BooleanLiteral(self.boolean_literal(false)),
                "null" => Literal::NullLiteral(self.null_literal()),
                _ => panic!("Unsupported Literal type {}", t.typ),
            }
        } else {
            panic!("Unexpected end of file")
        };
        return literal;
    }

    fn boolean_literal(&mut self, value: bool) -> BooleanLiteral<'a> {
        if value {
            self.eat("true");
            return BooleanLiteral {
                typ: "BooleanLiteral",
                value: true,
            };
        } else {
            self.eat("false");
            return BooleanLiteral {
                typ: "BooleanLiteral",
                value: false,
            };
        }
    }

    fn null_literal(&self) -> NullLiteral<'a> {
        return NullLiteral { typ: "NullLiteral" };
    }

    // NumericLiteral
    //  : STRING
    //  ;
    fn string_literal(&mut self) -> StringLiteral<'a> {
        let token: Token = self.eat("STRING");
        let value = token.value;
        return StringLiteral {
            typ: "StringLiteral",
            value: value[1..value.len() - 1].to_string(),
        };
    }

    // NumericLiteral
    //  : NUMBER
    //  ;
    fn numeric_literal(&mut self) -> NumericLiteral<'a> {
        let token: Token = self.eat("NUMBER");

        return NumericLiteral {
            typ: "NumericLiteral",
            value: token.value.parse::<i64>().unwrap(),
        };
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
