#[cfg(test)]
mod test {
    use letter_rdp_rust::*;
    #[test]
    fn handles_empty_statement() {
        let mut parser = init();

        let result = parser.parse(";");

        assert_eq!(
            result,
            Program {
                typ: "Program",
                body: vec![Statement::EmptyStatement {
                    typ: "EmptyStatement",
                }]
            }
        )
    }
}
