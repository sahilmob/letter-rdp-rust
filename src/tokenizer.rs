use super::nodes::*;
use regex::Regex;

const SPEC: [(&str, &str); 7] = [
    (r"^\s+", "WHITESPACE"),
    (r"^//.*", "COMMENT"),
    (r"^/*[\s\S]*?*/.*", "COMMENT"),
    (r"^;", ";"),
    (r"^\d+", "NUMBER"),
    (r#"^"[^"]*""#, "STRING"),
    (r#"^'[^']*'"#, "STRING"),
];
pub struct Tokenizer<'a> {
    pub string: &'a str,
    pub cursor: u64,
}

impl<'a> Tokenizer<'a> {
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

    pub fn get_next_token(&mut self) -> Option<Token<'a>> {
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
                        return self.get_next_token();
                    }
                    return Some(Token { typ: typ, value: v });
                }
                None => continue,
            };
        }

        panic!("Unexpected token: {}", string.chars().next().unwrap());
    }
}
