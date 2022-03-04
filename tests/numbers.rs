#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn prints_number() {
        let mut parser = init();

        let result = parser.parse::<i64>(String::from("  1"));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("NumericLiteral"),
                    value: 1
                }
            }
        )
    }
}
