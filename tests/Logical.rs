#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_logical_and() {
        let mut parser = init();

        let result = parser.parse("x > 0 && y < 1;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::LogicalExpression(LogicalExpression {
                        typ: "LogicalExpression",
                        operator: String::from("&&"),
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
                        right: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from("<"),
                            left: Box::new(Expression::LeftHandSideExpression(
                                LeftHandSideExpression::Identifier(Identifier {
                                    typ: "Identifier",
                                    name: String::from("y")
                                })
                            )),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 1
                                }
                            )))
                        }))
                    })
                })]
            }
        )
    }

    #[test]
    fn handles_logical_or() {
        let mut parser = init();

        let result = parser.parse("x > 0 || y < 1;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::LogicalExpression(LogicalExpression {
                        typ: "LogicalExpression",
                        operator: String::from("||"),
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
                        right: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from("<"),
                            left: Box::new(Expression::LeftHandSideExpression(
                                LeftHandSideExpression::Identifier(Identifier {
                                    typ: "Identifier",
                                    name: String::from("y")
                                })
                            )),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 1
                                }
                            )))
                        }))
                    })
                })]
            }
        )
    }
}
