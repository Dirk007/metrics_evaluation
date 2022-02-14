use anyhow::Result;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    Add, // +
    Sub, // -
    Div, // /
    Mul, // *
}

pub trait Calculateable: Sized {
    fn calculate(self, rhs: Self, operator: Arithmetic) -> Result<Self>;
}
