#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_block() {
        let mut parser = init();

        let result = parser.parse(
            "{
                \"hello\";
                42;
            }
            ",
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Box::new(BlockStatement {
                    typ: String::from("BlockStatement"),
                    body: vec![
                        Box::new(ExpressionStatement {
                            typ: String::from("ExpressionStatement"),
                            expression: Box::new(StringLiteral {
                                typ: String::from("StringLiteral"),
                                value: String::from("hello")
                            })
                        }),
                        Box::new(ExpressionStatement {
                            typ: String::from("ExpressionStatement"),
                            expression: Box::new(NumericLiteral {
                                typ: String::from("NumericLiteral"),
                                value: 42
                            })
                        })
                    ]
                })]
            }
        )
    }

    #[test]
    fn handles_empty_block() {
        let mut parser = init();

        let result = parser.parse("{}");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Box::new(BlockStatement {
                    typ: String::from("BlockStatement"),
                    body: vec![]
                })]
            }
        )
    }

    #[test]
    fn handles_nested_blocks() {
        let mut parser = init();

        let result = parser.parse(
            "{

                {
                    'hello';
                    42;
                }

        }",
        );

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Box::new(BlockStatement {
                    typ: String::from("BlockStatement"),
                    body: vec![Box::new(BlockStatement {
                        typ: String::from("BlockStatement"),
                        body: vec![
                            Box::new(ExpressionStatement {
                                typ: String::from("ExpressionStatement"),
                                expression: Box::new(StringLiteral {
                                    typ: String::from("StringLiteral"),
                                    value: String::from("hello")
                                })
                            }),
                            Box::new(ExpressionStatement {
                                typ: String::from("ExpressionStatement"),
                                expression: Box::new(NumericLiteral {
                                    typ: String::from("NumericLiteral"),
                                    value: 42
                                })
                            })
                        ]
                    })]
                })]
            }
        )
    }
}
