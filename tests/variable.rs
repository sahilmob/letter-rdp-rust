#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_single_variable_declaration() {
        let mut parser = init();

        let result = parser.parse("let x = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![VariableDeclaration {
                        id: Identifier {
                            typ: "Identifier",
                            name: String::from("x")
                        },
                        init: None
                    }]
                })]
            }
        )
    }

    #[test]
    fn handles_multiple_variable_declarations() {
        let mut parser = init();

        let result = parser.parse("let x, y;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![
                        VariableDeclaration {
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("x")
                            },
                            init: None
                        },
                        VariableDeclaration {
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("y")
                            },
                            init: None
                        }
                    ]
                })]
            }
        )
    }

    #[test]
    fn handles_multiple_variable_declarations_with_init() {
        let mut parser = init();

        let result = parser.parse("let x, y = 42;");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::VariableStatement(VariableStatement {
                    typ: "VariableStatement",
                    declarations: vec![
                        VariableDeclaration {
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("x")
                            },
                            init: None
                        },
                        VariableDeclaration {
                            id: Identifier {
                                typ: "Identifier",
                                name: String::from("y")
                            },
                            init: None
                        }
                    ]
                })]
            }
        )
    }
}
