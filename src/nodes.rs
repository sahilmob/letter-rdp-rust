#[derive(Debug, PartialEq)]
pub struct Program {
    pub value: NumericLiteral,
}

impl Program {
    pub const TYPE: &'static str = "Program";
}

#[derive(Debug, PartialEq)]
pub struct NumericLiteral {
    pub value: i64,
}

impl NumericLiteral {
    pub const TYPE: &'static str = "NumericLiteral";
}
