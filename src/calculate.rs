use anyhow::Result;

use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    Add, // +
    Sub, // -
    Div, // /
    Mul, // *
}

// FIXME: Append the calcs to the Comparison variable type and make everything nested in there

#[derive(Debug, PartialEq)]
pub enum Calculation {
    Value(Value, Arithmetic),
    Variable(String, Arithmetic),
}

pub trait Calculateable: Sized {
    fn calculate(self, rhs: &Self, operator: Arithmetic) -> Result<Self>;
}
