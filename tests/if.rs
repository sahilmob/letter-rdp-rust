#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_if_else_statement() {
        let mut parser = init();

        let result = parser.parse(
            "
            if (x) {
                x = 1;
            } else {
                x = 2;
            }
        ",
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::IfStatement(IfStatement {
                    typ: "IfStatement",
                    test: Test::Identifier(Identifier {
                        typ: "Identifier",
                        name: String::from("x")
                    }),
                    consequent: Box::new(Statement::BlockStatement(BlockStatement {
                        typ: "BlockStatement",
                        body: vec![Statement::ExpressionStatement(ExpressionStatement {
                            typ: "ExpressionStatement",
                            expression: Expression::AssignmentExpression(AssignmentExpression {
                                typ: "AssignmentExpression",
                                operator: String::from("="),
                                left: Identifier {
                                    typ: "Identifier",
                                    name: String::from("x")
                                },
                                right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                    NumericLiteral {
                                        typ: "NumericLiteral",
                                        value: 1
                                    }
                                )))
                            })
                        })]
                    })),
                    alternate: Some(Box::new(Statement::BlockStatement(BlockStatement {
                        typ: "BlockStatement",
                        body: vec![Statement::ExpressionStatement(ExpressionStatement {
                            typ: "ExpressionStatement",
                            expression: Expression::AssignmentExpression(AssignmentExpression {
                                typ: "AssignmentExpression",
                                operator: String::from("="),
                                left: Identifier {
                                    typ: "Identifier",
                                    name: String::from("x")
                                },
                                right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                    NumericLiteral {
                                        typ: "NumericLiteral",
                                        value: 2
                                    }
                                )))
                            })
                        })]
                    })))
                })]
            }
        )
    }

    #[test]
    fn handles_if_statement() {
        let mut parser = init();

        let result = parser.parse(
            "
            if (x) {
                x = 1;
            }
        ",
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::IfStatement(IfStatement {
                    typ: "IfStatement",
                    test: Test::Identifier(Identifier {
                        typ: "Identifier",
                        name: String::from("x")
                    }),
                    consequent: Box::new(Statement::BlockStatement(BlockStatement {
                        typ: "BlockStatement",
                        body: vec![Statement::ExpressionStatement(ExpressionStatement {
                            typ: "ExpressionStatement",
                            expression: Expression::AssignmentExpression(AssignmentExpression {
                                typ: "AssignmentExpression",
                                operator: String::from("="),
                                left: Identifier {
                                    typ: "Identifier",
                                    name: String::from("x")
                                },
                                right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                    NumericLiteral {
                                        typ: "NumericLiteral",
                                        value: 1
                                    }
                                )))
                            })
                        })]
                    })),
                    alternate: None
                })]
            }
        )
    }

    #[test]
    fn handles_if_with_statement_as_consequent() {
        let mut parser = init();

        let result = parser.parse(
            "
            if (x) x = 1;
        ",
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::IfStatement(IfStatement {
                    typ: "IfStatement",
                    test: Test::Identifier(Identifier {
                        typ: "Identifier",
                        name: String::from("x")
                    }),
                    consequent: Box::new(Statement::ExpressionStatement(ExpressionStatement {
                        typ: "ExpressionStatement",
                        expression: Expression::AssignmentExpression(AssignmentExpression {
                            typ: "AssignmentExpression",
                            operator: String::from("="),
                            left: Identifier {
                                typ: "Identifier",
                                name: String::from("x")
                            },
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 1
                                }
                            )))
                        })
                    })),
                    alternate: None
                })]
            }
        )
    }

    #[test]
    fn handles_nested_if_statements() {
        let mut parser = init();

        let result = parser.parse(
            "
            if (x) if (y) {} else {}
        ",
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::IfStatement(IfStatement {
                    typ: "IfStatement",
                    test: Test::Identifier(Identifier {
                        typ: "Identifier",
                        name: String::from("x")
                    }),
                    consequent: Box::new(Statement::IfStatement(IfStatement {
                        typ: "IfStatement",
                        test: Test::Identifier(Identifier {
                            typ: "Identifier",
                            name: String::from("y")
                        }),
                        consequent: Box::new(Statement::BlockStatement(BlockStatement {
                            typ: "BlockStatement",
                            body: vec![]
                        })),
                        alternate: Some(Box::new(Statement::BlockStatement(BlockStatement {
                            typ: "BlockStatement",
                            body: vec![]
                        })))
                    })),
                    alternate: None
                })]
            }
        )
    }
}
