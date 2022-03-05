#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_multiple_statements() {
        let mut parser = init();

        let result = parser.parse(String::from(
            r##"  
        1;
        "hello";
        "##,
        ));

        assert_eq!(
            result,
            Program {
                typ: String::from("Program"),
                body: vec![
                    ExpressionStatement {
                        typ: String::from("ExpressionStatement"),
                        expression: Literal::NumericLiteral {
                            typ: String::from("NumericLiteral"),
                            value: 1
                        }
                    },
                    ExpressionStatement {
                        typ: String::from("ExpressionStatement"),
                        expression: Literal::StringLiteral {
                            typ: String::from("StringLiteral"),
                            value: String::from("hello")
                        }
                    }
                ]
            }
        )
    }
}
