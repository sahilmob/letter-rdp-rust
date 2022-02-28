#[derive(Debug, PartialEq)]
pub struct NumericLiteral {
    pub value: i64,
}

impl NumericLiteral {
    pub const TYPE: &'static str = "NumericLiteral";
}
