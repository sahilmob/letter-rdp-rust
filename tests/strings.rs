#[cfg(test)]
mod test {
    use letter_rdp_rust::*;

    #[test]
    fn prints_double_quotes_string() {
        let mut parser = init();

        let result = parser.parse::<String>(String::from(r#""a""#));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }

    #[test]
    fn prints_single_quotes_string() {
        let mut parser = init();

        let result = parser.parse::<String>(String::from(r#"   'a'"#));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }

    #[test]
    fn ignores_single_line_comments() {
        let mut parser = init();

        let result = parser.parse::<String>(String::from(
            r#"   
            // comment
        'a'
        "#,
        ));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }

    #[test]
    fn ignores_multiline_comments() {
        let mut parser = init();

        let result = parser.parse::<String>(String::from(
            r#"   
            /*
            *  a comment
            */
        'a'
        "#,
        ));

        assert_eq!(
            result,
            Program {
                value: Literal {
                    typ: String::from("StringLiteral"),
                    value: String::from("a")
                }
            }
        )
    }
}