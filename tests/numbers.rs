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
                typ: String::from("Program"),
                body: vec![ExpressionStatement {
                    typ: String::from("ExpressionStatement"),
                    expression: Literal::NumericLiteral {
                        typ: String::from("NumericLiteral"),
                        value: 1
                    }
                }]
            }
        )
    }
}
