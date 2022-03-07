#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn binary_expression() {
        let mut parser = init();

        let result = parser.parse("2+2;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("+"),
                        left: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        ))),
                        right: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn chained_binary_expression() {
        let mut parser = init();

        let result = parser.parse("3 + 2 - 2;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("-"),
                        left: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from("+"),
                            left: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 3
                                }
                            ))),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 2
                                }
                            )))
                        })),
                        right: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn multiplicative_operator() {
        let mut parser = init();

        let result = parser.parse("2 * 2;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("*"),
                        right: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        ))),
                        left: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn chained_binary_expression_with_multiplication() {
        let mut parser = init();

        let result = parser.parse("2 + 2 * 2;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("+"),
                        right: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from("*"),
                            left: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 2
                                }
                            ))),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 2
                                }
                            )))
                        })),
                        left: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn parenthesized_expression() {
        let mut parser = init();

        let result = parser.parse("(2 + 2) * 2;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from("*"),
                        left: Box::new(Expression::BinaryExpression(BinaryExpression {
                            typ: "BinaryExpression",
                            operator: String::from("+"),
                            left: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 2
                                }
                            ))),
                            right: Box::new(Expression::Literal(Literal::NumericLiteral(
                                NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 2
                                }
                            )))
                        })),
                        right: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 2
                            }
                        )))
                    })
                })]
            }
        )
    }
}
