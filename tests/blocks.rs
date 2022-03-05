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
                body: vec![Statement::BlockStatement {
                    typ: "BlockStatement",
                    body: vec![
                        Statement::ExpressionStatement {
                            typ: "ExpressionStatement",
                            expression: Literal::StringLiteral {
                                typ: "StringLiteral",
                                value: String::from("hello")
                            }
                        },
                        Statement::ExpressionStatement {
                            typ: "ExpressionStatement",
                            expression: Literal::NumericLiteral {
                                typ: "NumericLiteral",
                                value: 42
                            }
                        }
                    ]
                }]
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
                body: vec![Statement::BlockStatement {
                    typ: "BlockStatement",
                    body: vec![]
                }]
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
                body: vec![Statement::BlockStatement {
                    typ: "BlockStatement",
                    body: vec![Statement::BlockStatement {
                        typ: "BlockStatement",
                        body: vec![
                            Statement::ExpressionStatement {
                                typ: "ExpressionStatement",
                                expression: Literal::StringLiteral {
                                    typ: "StringLiteral",
                                    value: String::from("hello")
                                }
                            },
                            Statement::ExpressionStatement {
                                typ: "ExpressionStatement",
                                expression: Literal::NumericLiteral {
                                    typ: "NumericLiteral",
                                    value: 42
                                }
                            }
                        ]
                    }]
                }]
            }
        )
    }
}
