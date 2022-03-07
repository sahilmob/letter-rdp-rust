#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_multiple_statements() {
        let mut parser = init();

        let result = parser.parse(
            r##"  
        1;
        "hello";
        "##,
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![
                    Statement::ExpressionStatement(ExpressionStatement {
                        typ: "ExpressionStatement",
                        expression: Expression::Literal(Literal::NumericLiteral(NumericLiteral {
                            typ: "NumericLiteral",
                            value: 1
                        }))
                    }),
                    Statement::ExpressionStatement(ExpressionStatement {
                        typ: "ExpressionStatement",
                        expression: Expression::Literal(Literal::StringLiteral(StringLiteral {
                            typ: "StringLiteral",
                            value: String::from("hello")
                        }))
                    })
                ]
            }
        )
    }
}
