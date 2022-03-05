#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn prints_number() {
        let mut parser = init();

        let result = parser.parse(String::from("  1;"));

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![ExpressionStatement {
                    typ: "ExpressionStatement",
                    expression: Literal::NumericLiteral {
                        typ: "NumericLiteral",
                        value: 1
                    }
                }]
            }
        )
    }
}
