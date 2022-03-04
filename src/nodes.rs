use std::str::FromStr;

#[derive(Debug, PartialEq)]

pub struct Literal<T: FromStr> {
    pub typ: String,
    pub value: T,
}

#[derive(Debug, PartialEq)]
pub struct Program<T: FromStr> {
    pub value: Literal<T>,
}

impl<T: FromStr> Program<T> {
    pub const typ: &'static str = "Program";
}

#[derive(Debug, PartialEq)]
pub struct NumericLiteral {
    pub value: i64,
}

// impl NumericLiteral {
//     pub const typ: &'static str = "NumericLiteral";
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub typ: String,
    pub value: String,
}
