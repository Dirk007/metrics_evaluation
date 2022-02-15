use anyhow::Result;

use crate::value::Value;

/// Enumerations for arethmetic opersions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    /// `+` -> addition
    Add,
    /// `-` -> substraction
    Sub,
    /// `/` -> division
    Div,
    /// `*` -> multiplication
    Mul,
}

/// Encapsulates a calculation on a given [Value] or a dynamically [crate::Resolver::resolve]d value.
#[derive(Debug, PartialEq)]
pub enum Calculation {
    Value(Value, Arithmetic),
    Variable(String, Arithmetic),
}

/// Trait to guarantee that a given type is calculateable with [Arithmetic]
pub trait Calculateable: Sized {
    fn calculate(self, rhs: &Self, operator: Arithmetic) -> Result<Self>;
}
