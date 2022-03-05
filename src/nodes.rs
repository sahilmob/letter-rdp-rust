#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub typ: &'a str,
    pub body: Vec<ExpressionStatement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub typ: &'a str,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement<'a> {
    pub typ: &'a str,
    pub expression: Literal<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    NumericLiteral { typ: &'a str, value: i64 },
    StringLiteral { typ: &'a str, value: String },
}
