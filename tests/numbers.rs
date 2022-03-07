#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn prints_number() {
        let mut parser = init();

        let result = parser.parse("  1;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::ExpressionStatement(ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Expression::Literal(Literal::NumericLiteral(NumericLiteral {
                        typ: "NumericLiteral",
                        value: 1
                    }))
                })]
            }
        )
    }
}
