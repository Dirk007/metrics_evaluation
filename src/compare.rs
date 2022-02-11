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

/// Defines if a comparison is against a value or against another variable
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonType {
    /// A comparison of a variable against a fixed value
    Value(ValueComparison),
    /// A comparison of a variable against an other variable
    Variable(VariableComparison),
}

/// A simple comparison of a variable `name` against a `value`
#[derive(Debug, Clone, PartialEq)]
pub struct ValueComparison {
    /// Name of the variable
    pub name: String,
    /// [Operator] to use for the comparison
    pub operator: Operator,
    /// [Value] to compare the content of `name` to
    pub value: Value,
}

/// A simple comparison of a variable `lhs` against the content of another variable `rhs`
#[derive(Debug, Clone, PartialEq)]
pub struct VariableComparison {
    /// Name of the variable
    pub lhs: String,
    /// [Operator] to use for the comparison
    pub operator: Operator,
    /// Name of the other variable
    pub rhs: String,
}

/// Triplet to [ValueComparison] conversion
impl From<(&str, Operator, Value)> for ValueComparison {
    fn from((name, op, value): (&str, Operator, Value)) -> Self {
        Self::new(name, op, value)
    }
}

impl ValueComparison {
    pub fn new(name: impl AsRef<str>, operator: Operator, value: Value) -> Self {
        Self {
            name: name.as_ref().into(),
            operator,
            value,
        }
    }
}

/// Triplet to [VariableComparison] conversion
impl From<(&str, Operator, &str)> for VariableComparison {
    fn from((lhs, op, rhs): (&str, Operator, &str)) -> Self {
        Self::new(lhs, op, rhs)
    }
}

impl VariableComparison {
    pub fn new(name: impl AsRef<str>, operator: Operator, rhs_name: impl AsRef<str>) -> Self {
        Self {
            lhs: name.as_ref().into(),
            operator,
            rhs: rhs_name.as_ref().into(),
        }
    }
}

/// Converts a triplet of [str], [Operator], [str] to a ComparisonType of [ComparisonType::Variable]
impl From<(&str, Operator, &str)> for ComparisonType {
    fn from((lhs, op, rhs): (&str, Operator, &str)) -> Self {
        Self::Variable((lhs, op, rhs).into())
    }
}

/// Converts a triplet of [str], [Operator], [Value] to a ComparisonType of [ComparisonType::Value]
impl From<(&str, Operator, Value)> for ComparisonType {
    fn from((name, op, value): (&str, Operator, Value)) -> Self {
        Self::Value((name, op, value).into())
    }
}
