use std::{
    ops::{Add, Div, Mul, Sub},
    time::Duration,
};

use anyhow::{bail, Result};
use chrono::naive::NaiveTime;

use crate::{
    calculate::{Arithmetic, Calculateable},
    compare::{Compareable, Operator},
};

/// Representation of different value types for use in comparisons.
/// Is [Compareable] and [Calculateable].
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    /// A string-value which represents any kind of text sequence
    String(String),
    /// A numeric value which has u8 to f64 conversions implemented
    Numeric(f64),
    /// A simple [bool] wrapper
    Bool(bool),
    /// A time-representation
    Time(NaiveTime),
    /// A duration in time
    Duration(Duration),
}

impl Compareable for Value {
    fn compare(&self, rhs: &Self, operator: Operator) -> bool {
        match operator {
            Operator::Equal => self == rhs,
            Operator::NotEqual => self != rhs,
            Operator::Greater => self > rhs,
            Operator::Less => self < rhs,
            Operator::GreaterEqual => self >= rhs,
            Operator::LessEqual => self <= rhs,
        }
    }
}

/// ```
/// use metrics_evaluation::*;
///
/// let foo = Value::Numeric(1.0);
/// let bar = foo
///     .calculate(&Value::Numeric(2.0), Arithmetic::Add)
///     .unwrap();
/// assert_eq!(bar, Value::Numeric(3.0));
///
/// let foo = Value::Numeric(4.0);
/// let bar = foo
///     .calculate(&Value::Numeric(3.0), Arithmetic::Sub)
///     .unwrap();
/// assert_eq!(bar, Value::Numeric(1.0));
///
/// let foo = Value::Numeric(4.0);
/// let bar = foo
///     .calculate(&Value::Numeric(2.0), Arithmetic::Mul)
///     .unwrap();
/// assert_eq!(bar, Value::Numeric(8.0));
///
/// let foo = Value::Numeric(4.0);
/// let bar = foo
///     .calculate(&Value::Numeric(2.0), Arithmetic::Div)
///     .unwrap();
/// assert_eq!(bar, Value::Numeric(2.0));
/// ```
impl Calculateable for Value {
    fn calculate(self, rhs: &Self, operator: Arithmetic) -> Result<Self> {
        match operator {
            Arithmetic::Add => self + rhs,
            Arithmetic::Sub => self - rhs,
            Arithmetic::Mul => self * rhs,
            Arithmetic::Div => self / rhs,
        }
    }
}

impl Add<&Self> for Value {
    type Output = Result<Value>;

    fn add(self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs + &rhs)),
            (Value::String(lhs), Value::Numeric(rhs)) => {
                Ok(Value::String(format!("{} {}", lhs, rhs)))
            }
            (Value::Numeric(lhs), Value::Numeric(rhs)) => Ok(Value::Numeric(lhs + rhs)),
            (Value::Duration(lhs), Value::Duration(rhs)) => Ok(Value::Duration(lhs + *rhs)),
            (Value::Time(lhs), Value::Duration(rhs)) => Ok(Value::Time(
                lhs + chrono::Duration::from_std(rhs.clone()).expect("Unable to convert duration"),
            )),
            _ => bail!("Incompatible types for addition"),
        }
    }
}

impl Sub<&Self> for Value {
    type Output = Result<Value>;

    fn sub(self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Value::Numeric(lhs), Value::Numeric(rhs)) => Ok(Value::Numeric(lhs - rhs)),
            (Value::Duration(lhs), Value::Duration(rhs)) => Ok(Value::Duration(lhs - *rhs)),
            (Value::Time(lhs), Value::Duration(rhs)) => Ok(Value::Time(
                lhs - chrono::Duration::from_std(rhs.clone()).expect("Unable to convert duration"),
            )),
            _ => bail!("Incompatible types for substraction"),
        }
    }
}

impl Mul<&Self> for Value {
    type Output = Result<Value>;

    fn mul(self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Value::Numeric(lhs), Value::Numeric(rhs)) => Ok(Value::Numeric(lhs * rhs)),
            (Value::Duration(lhs), Value::Numeric(rhs)) => {
                Ok(Value::Duration(lhs * rhs.round() as u32))
            }
            _ => bail!("Incompatible types for multiiplication"),
        }
    }
}

impl Div<&Self> for Value {
    type Output = Result<Value>;

    fn div(self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Value::Numeric(lhs), Value::Numeric(rhs)) => Ok(Value::Numeric(lhs / rhs)),
            (Value::Duration(lhs), Value::Numeric(rhs)) => {
                Ok(Value::Duration(lhs / rhs.round() as u32))
            }
            _ => bail!("Incompatible types for division"),
        }
    }
}

macro_rules! impl_value {
    ($from:ty, $to:expr) => {
        impl From<$from> for Value {
            fn from(value: $from) -> Self {
                $to(value.into())
            }
        }
    };
}

/// One lazy helper
impl TryFrom<chrono::Duration> for Value {
    type Error = anyhow::Error;

    fn try_from(value: chrono::Duration) -> Result<Self, Self::Error> {
        Ok(value.to_std()?.into())
    }
}

impl_value!(&str, Value::String);
impl_value!(String, Value::String);
impl_value!(i8, Value::Numeric);
impl_value!(u8, Value::Numeric);
impl_value!(i16, Value::Numeric);
impl_value!(u16, Value::Numeric);
impl_value!(i32, Value::Numeric);
impl_value!(u32, Value::Numeric);
impl_value!(f32, Value::Numeric);
impl_value!(f64, Value::Numeric);
impl_value!(bool, Value::Bool);
impl_value!(NaiveTime, Value::Time);
impl_value!(Duration, Value::Duration);
