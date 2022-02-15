use crate::{value::Value, Calculation};

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
#[derive(Debug, PartialEq)]
pub enum ComparisonType {
    /// A comparison of a variable against a fixed value (wich optional calculations)
    Value(Value, Vec<Calculation>),
    /// A comparison of a variable against an other variable (wich optional calculations)
    Variable(String, Vec<Calculation>),
}

impl ComparisonType {
    pub fn with_calculation(&mut self, calc: Calculation) {
        match self {
            Self::Value(_, calcs) => calcs.push(calc),
            Self::Variable(_, calcs) => calcs.push(calc),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Comparison {
    /// Left-Hand-Side of the comparison (which the rhs will be compared to)
    pub lhs: ComparisonType,
    /// [Operator] to use for the comparison
    pub operator: Operator,
    /// Right-Hand-Side to compare the content of `lhs` to
    pub rhs: ComparisonType,
}

/// Triplet to [Comparison] conversion
impl From<(&str, Operator, Value)> for Comparison {
    fn from((name, operator, value): (&str, Operator, Value)) -> Self {
        Self {
            lhs: ComparisonType::Variable(name.into(), Vec::new()),
            operator,
            rhs: ComparisonType::Value(value, Vec::new()),
        }
    }
}

/// Triplet to [Comparison] conversion
impl From<(&str, Operator, &str)> for Comparison {
    fn from((name, operator, rhs): (&str, Operator, &str)) -> Self {
        Self {
            lhs: ComparisonType::Variable(name.into(), Vec::new()),
            operator,
            rhs: ComparisonType::Variable(rhs.into(), Vec::new()),
        }
    }
}
