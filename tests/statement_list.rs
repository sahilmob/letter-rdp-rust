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
                    Box::new(ExpressionStatement {
                        typ: String::from("ExpressionStatement"),
                        expression: Box::new(NumericLiteral {
                            typ: String::from("NumericLiteral"),
                            value: 1
                        })
                    }),
                    Box::new(ExpressionStatement {
                        typ: String::from("ExpressionStatement"),
                        expression: Box::new(StringLiteral {
                            typ: String::from("StringLiteral"),
                            value: String::from("hello")
                        })
                    })
                ]
            }
        )
    }
}
