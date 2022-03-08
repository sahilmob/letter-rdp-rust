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
                body: vec![Box::new(ExpressionStatement {
                    typ: String::from("ExpressionStatement"),
                    expression: Box::new(NumericLiteral {
                        typ: String::from("NumericLiteral"),
                        value: 1
                    })
                })]
            }
        )
    }
}
