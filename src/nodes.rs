// pub struct Literal<T: FromStr> {
//     pub typ: String,
//     pub value: T,
// }

#[derive(Debug, PartialEq)]
pub enum Literal {
    NumericLiteral { typ: String, value: i64 },
    StringLiteral { typ: String, value: String },
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub typ: String,
    pub body: Vec<ExpressionStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub typ: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub typ: String,
    pub expression: Literal,
}
