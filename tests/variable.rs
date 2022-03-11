#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_single_variable_declaration() {
        let mut parser = init();

        let result = parser.parse("let x = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![VariableDeclaration {
                        typ: "VariableDeclaration",
                        id: Identifier {
                            typ: "Identifier",
                            name: String::from("x")
                        },
                        init: Some(VariableInitializer::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 42
                            }
                        )))
                    }]
                })]
            }
        )
    }

    #[test]
    fn handles_multiple_variable_declarations() {
        let mut parser = init();

        let result = parser.parse("let x, y;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![
                        VariableDeclaration {
                            typ: "VariableDeclaration",
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("x")
                            },
                            init: None
                        },
                        VariableDeclaration {
                            typ: "VariableDeclaration",
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("y")
                            },
                            init: None
                        }
                    ]
                })]
            }
        )
    }

    #[test]
    fn handles_multiple_variable_declarations_with_init() {
        let mut parser = init();

        let result = parser.parse("let x, y = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![
                        VariableDeclaration {
                            typ: "VariableDeclaration",
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("x")
                            },
                            init: None
                        },
                        VariableDeclaration {
                            typ: "VariableDeclaration",
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("y")
                            },
                            init: Some(VariableInitializer::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 42
                                }
                            )))
                        }
                    ]
                })]
            }
        )
    }

    #[test]
    fn handles_single_variable_declaration_with_assignment_expr_as_init() {
        let mut parser = init();

        let result = parser.parse("let x = y = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![VariableDeclaration {
                        typ: "VariableDeclaration",
                        id: Identifier {
                            typ: "Identifier",
                            name: String::from("x")
                        },
                        init: Some(VariableInitializer::AssignmentExpression(
                            AssignmentExpression {
                                typ: "AssignmentExpression",
                                operator: String::from("="),
                                left: Identifier {
                                    typ: "Identifier",
                                    name: String::from("y")
                                },
                                right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                    NumericLiteral {
                                        typ: "NumericLiteral",
                                        value: 42
                                    }
                                )))
                            }
                        ))
                    },]
                })]
            }
        )
    }
}
