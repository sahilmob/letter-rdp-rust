#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_assignment_expression() {
        let mut parser = init();

        let result = parser.parse("x = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
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
                                value: 42
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn handles_multiple_assignment_expression() {
        let mut parser = init();

        let result = parser.parse("x = y = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::AssignmentExpression(AssignmentExpression {
                        typ: "AssignmentExpression",
                        operator: String::from("="),
                        left: Identifier {
                            typ: "Identifier",
                            name: String::from("x")
                        },
                        right: Box::new(Expression::AssignmentExpression(AssignmentExpression {
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
                        }))
                    })
                })]
            }
        )
    }
}
