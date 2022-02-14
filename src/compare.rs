use crate::value::Value;
use crate::Calculation;

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
    Value(Value),
    /// A comparison of a variable against an other variable
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub struct Comparison {
    /// Name of the variable
    pub name: String,
    /// [Operator] to use for the comparison
    pub operator: Operator,
    /// Thing to compare the content of `name` to
    pub comparison_type: ComparisonType,
    /// Optional arithmetics on rhs
    pub calculations: Vec<Calculation>,
}

impl Comparison {
    pub fn with_calculation(mut self, calc: Calculation) -> Self {
        self.calculations.push(calc);
        self
    }
}

/// Triplet to [ValueComparison] conversion
impl From<(&str, Operator, Value)> for Comparison {
    fn from((name, operator, value): (&str, Operator, Value)) -> Self {
        Self {
            name: name.into(),
            operator,
            comparison_type: ComparisonType::Value(value),
            calculations: Vec::new(),
        }
    }
}

/// Triplet to [VariableComparison] conversion
impl From<(&str, Operator, &str)> for Comparison {
    fn from((name, operator, rhs): (&str, Operator, &str)) -> Self {
        Self {
            name: name.into(),
            operator,
            comparison_type: ComparisonType::Variable(rhs.into()),
            calculations: Vec::new(),
        }
    }
}
