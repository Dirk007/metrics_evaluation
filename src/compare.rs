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
    /// Compare self with `against` using the given [Operator]
    fn compare(&self, other: &Self, operator: Operator) -> bool;
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
    pub fn with_calculation(&mut self, calculation: Calculation) {
        match self {
            Self::Value(_, calculations) => calculations.push(calculation),
            Self::Variable(_, calculations) => calculations.push(calculation),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Comparison {
    /// Left-Hand-Side of the comparison (which the rhs will be compared to)
    pub what: ComparisonType,
    /// [Operator] to use for the comparison
    pub operator: Operator,
    /// Right-Hand-Side to compare the content of `lhs` to
    pub against: ComparisonType,
}

/// Triplet (variable-name, operator, value) to [Comparison] conversion
impl From<(&str, Operator, Value)> for Comparison {
    fn from((variable_name, operator, value): (&str, Operator, Value)) -> Self {
        Self {
            what: ComparisonType::Variable(variable_name.into(), Vec::new()),
            operator,
            against: ComparisonType::Value(value, Vec::new()),
        }
    }
}

/// Triplet (variable-name, operator, variable-name) to [Comparison] conversion
impl From<(&str, Operator, &str)> for Comparison {
    fn from((variable_name, operator, against_variable_name): (&str, Operator, &str)) -> Self {
        Self {
            what: ComparisonType::Variable(variable_name.into(), Vec::new()),
            operator,
            against: ComparisonType::Variable(against_variable_name.into(), Vec::new()),
        }
    }
}
