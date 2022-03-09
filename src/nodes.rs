#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub typ: &'a str,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub typ: &'a str,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    ExpressionStatement(ExpressionStatement<'a>),
    BlockStatement(BlockStatement<'a>),
    EmptyStatement { typ: &'a str },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement<'a> {
    pub typ: &'a str,
    pub expression: Expression<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement<'a> {
    pub typ: &'a str,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Literal(Literal<'a>),
    BinaryExpression(BinaryExpression<'a>),
    AssignmentExpression(AssignmentExpression<'a>),
    LeftHandSideExpression(LeftHandSideExpression<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeftHandSideExpression<'a> {
    Identifier(Identifier<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a> {
    pub typ: &'a str,
    pub operator: String,
    pub left: Box<Expression<'a>>,
    pub right: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    NumericLiteral(NumericLiteral<'a>),
    StringLiteral(StringLiteral<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpression<'a> {
    pub typ: &'a str,
    pub operator: String,
    pub left: Identifier<'a>,
    pub right: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a> {
    pub typ: &'a str,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteral<'a> {
    pub typ: &'a str,
    pub value: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral<'a> {
    pub typ: &'a str,
    pub value: String,
}
