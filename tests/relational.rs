#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_relational_gt_expression() {
        let mut parser = init();

        let result = parser.parse("x > 0;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from(">"),
                        left: Box::new(Expression::LeftHandSideExpression(
                            LeftHandSideExpression::Identifier(Identifier {
                                typ: "Identifier",
                                name: String::from("x"),
                            })
                        )),
                        right: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 0
                            }
                        )))
                    })
                })]
            }
        )
    }

    #[test]
    fn handles_relational_gte_expression() {
        let mut parser = init();

        let result = parser.parse("x >= 0;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::BinaryExpression(BinaryExpression {
                        typ: "BinaryExpression",
                        operator: String::from(">="),
                        left: Box::new(Expression::LeftHandSideExpression(
                            LeftHandSideExpression::Identifier(Identifier {
                                typ: "Identifier",
                                name: String::from("x"),
                            })
                        )),
                        right: Box::new(Expression::Literal(Literal::NumericLiteral(
                            NumericLiteral {
                                typ: "NumericLiteral",
                                value: 0
                            }
                        )))
                    })
                })]
            }
        )
    }
}
