// pub struct Literal<T: FromStr> {
//     pub typ: String,
//     pub value: T,
// }

#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    NumericLiteral { typ: &'a str, value: i64 },
    StringLiteral { typ: &'a str, value: String },
}

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub typ: &'a str,
    pub body: Vec<ExpressionStatement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub typ: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement<'a> {
    pub typ: &'a str,
    pub expression: Literal<'a>,
}
