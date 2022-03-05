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
                typ: "Program",
                body: vec![
                    ExpressionStatement {
                        typ: "ExpressionStatement",
                        expression: Literal::NumericLiteral {
                            typ: "NumericLiteral",
                            value: 1
                        }
                    },
                    ExpressionStatement {
                        typ: "ExpressionStatement",
                        expression: Literal::StringLiteral {
                            typ: "StringLiteral",
                            value: String::from("hello")
                        }
                    }
                ]
            }
        )
    }
}
