use core::any::Any;
use std::fmt::Debug;

pub trait LiteralT {
    fn box_eq(&self, other: &dyn Any) -> bool;

    fn as_any(&self) -> &dyn Any;
}
pub trait StatementT: Any {
    fn box_eq(&self, other: &dyn Any) -> bool;

    fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn StatementT {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub typ: &'a str,
    pub body: Vec<Box<dyn StatementT>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub typ: &'a str,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]

pub struct NumericLiteral {
    pub typ: String,
    pub value: i64,
}
pub struct StringLiteral {
    pub typ: String,
    pub value: String,
}

impl LiteralT for StringLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Self>()
            .map_or(false, |a| self.value == a.value)
    }
}
impl LiteralT for NumericLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Self>()
            .map_or(false, |a| self.value == a.value)
    }
}

pub struct ExpressionStatement {
    pub typ: String,
    pub expression: Box<dyn LiteralT>,
}
#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub typ: String,
    pub body: Vec<Box<dyn StatementT>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct EmptyStatement {
    pub typ: String,
}

impl StatementT for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Self>()
            .map_or(false, |a| &self.expression == &a.expression)
    }
}
impl StatementT for BlockStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
}
impl StatementT for EmptyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Self>()
            .map_or_else(|| panic!("Not equal"), |a| self == a)
    }
}

impl PartialEq for Box<dyn StatementT> {
    fn eq(&self, other: &Box<dyn StatementT>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl PartialEq for Box<dyn LiteralT> {
    fn eq(&self, other: &Box<dyn LiteralT>) -> bool {
        self.box_eq(other.as_any())
    }
}
