#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_equality_expression() {
        let mut parser = init();

        let result = parser.parse("x > 0 == true;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("=="),
                        left: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from(">"),
                            left: Box::new(Expression::LeftHandSideExpression(
                                LeftHandSideExpression::Identifier(Identifier {
                                    typ: "Identifier",
                                    name: String::from("x")
                                })
                            )),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 0
                                }
                            )))
                        })),
                        right: Box::new(Expression::Literal(Literal::BooleanLiteral(
                            BooleanLiteral {
                                typ: "BooleanLiteral",
                                value: true
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn handles_negation_expression() {
        let mut parser = init();

        let result = parser.parse("x > 0 != true;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("!="),
                        left: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from(">"),
                            left: Box::new(Expression::LeftHandSideExpression(
                                LeftHandSideExpression::Identifier(Identifier {
                                    typ: "Identifier",
                                    name: String::from("x")
                                })
                            )),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 0
                                }
                            )))
                        })),
                        right: Box::new(Expression::Literal(Literal::BooleanLiteral(
                            BooleanLiteral {
                                typ: "BooleanLiteral",
                                value: true
                            }
                        )))
                    })
                })]
            }
        )
    }
}
