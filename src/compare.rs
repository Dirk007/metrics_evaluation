use crate::value::Value;

/// Logic for comparisons
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Logic {
    And,
    Or,
}

/// Comparison-operators
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Equal,        // ==
    NotEqual,     // !=
    GreaterEqual, // >=
    LessEqual,    // <=
    Greater,      // >
    Less,         // <
}

/// Helper to bind `compare` with our custom [Operator] to a struct
pub trait Compareable {
    /// Compare self with rhs using the given [Operator]
    fn compare(&self, rhs: &Self, operator: Operator) -> bool;
}

/// A simple comparison
#[derive(Debug, Clone)]
pub struct Comparison {
    /// Name of the variable
    pub name: String,
    /// [Operator] to use for the comparison
    pub operator: Operator,
    /// [Value] to compare the content of `name` to
    pub value: Value,
}

/// Triplet to [Comparison] conversion
impl From<(&str, Operator, Value)> for Comparison {
    fn from((name, op, value): (&str, Operator, Value)) -> Self {
        Comparison::new(name, op, value)
    }
}

impl Comparison {
    pub fn new(name: impl AsRef<str>, operator: Operator, value: Value) -> Self {
        Self {
            name: name.as_ref().into(),
            operator,
            value,
        }
    }
}
