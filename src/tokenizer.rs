use super::nodes::*;
use regex::Regex;

const SPEC: [(&str, &str); 18] = [
    (r"^\s+", "WHITESPACE"),
    (r"^//.*", "COMMENT"),
    (r"^/*[\s\S]*?*/.*", "COMMENT"),
    (r"^;", ";"),
    (r"^\{", "{"),
    (r"^\}", "}"),
    (r"^\(", "("),
    (r"^\)", ")"),
    (r"^,", ","),
    (r"^\blet\b", "let"),
    (r"^\d+", "NUMBER"),
    (r"^\w+", "IDENTIFIER"),
    (r"^=", "SIMPLE_ASSIGN"),
    (r"^[\*/\+\-]=", "COMPLEX_ASSIGN"),
    (r"^[+\-]", "ADDITIVE_OPERATOR"),
    (r"^[*\\/]", "MULTIPLICATIVE_OPERATOR"),
    (r#"^"[^"]*""#, "STRING"),
    (r#"^'[^']*'"#, "STRING"),
];
#[derive(Default, Debug)]
pub struct Tokenizer<'a> {
    pub string: &'a str,
    pub cursor: u64,
}

impl<'a> Tokenizer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, string: &'a str) {
        self.string = string;
    }

    fn has_more_tokens(&self) -> bool {
        return self.cursor < self.string.len().try_into().unwrap();
    }

    fn match_regexp(&mut self, regexp: &str, string: &str) -> Option<String> {
        let re = Regex::new(regexp).unwrap();
        return match re.captures(&string) {
            Some(v) => {
                let matched = v.get(0).map_or("", |m| m.as_str()).to_string();
                self.cursor += matched.len() as u64;
                return Some(matched);
            }
            None => None,
        };
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_more_tokens() {
            return None;
        }
        let idx = self.cursor as usize;
        let string = self.string[idx..].to_string();

        for (regexp, typ) in SPEC {
            let token_value = self.match_regexp(regexp, &string);
            match token_value {
                Some(v) => {
                    if typ == "WHITESPACE" || typ == "COMMENT" {
                        return self.next();
                    }
                    return Some(Token { typ: typ, value: v });
                }
                None => continue,
            };
        }

        panic!("Unexpected token: {}", string.chars().next().unwrap());
    }
}
