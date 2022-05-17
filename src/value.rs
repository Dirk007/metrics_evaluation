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
#[cfg_attr(not(feature = "lax_comparison"), derive(PartialEq))]
#[derive(Debug, PartialOrd, Clone)]
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

#[cfg(feature = "lax_comparison")]
/// Extra lax comparison tries to perform some conversions on compare and may succeed where std [PartialEq] fails
/// ```
/// use metrics_evaluation::*;
///
/// // Numeric vs Bool: anything != 0 is true
/// assert_eq!(&Value::Numeric(1.0), &Value::Bool(true));
/// assert_eq!(&Value::Numeric(2.0), &Value::Bool(true));
/// assert_eq!(&Value::Numeric(0.0), &Value::Bool(false));
/// assert_eq!(&Value::Bool(false), &Value::Numeric(0.0));
///
/// // Numeric vs. String
/// assert_eq!(&Value::String("1".into()), &Value::Numeric(1.0));
/// assert_eq!(&Value::Numeric(42.0), &Value::String("42".into()));
/// assert_ne!(&Value::Numeric(123.0), &Value::String("foo".into()));
///
/// // Bool vs. String
/// assert_eq!(&Value::String("true".into()), &Value::Bool(true));
/// assert_eq!(&Value::Bool(true), &Value::String("true".into()));
/// assert_eq!(&Value::String("false".into()), &Value::Bool(false));
/// assert_ne!(&Value::String("false".into()), &Value::Bool(true));
/// assert_ne!(&Value::String("foo".into()), &Value::Bool(true));
/// assert_ne!(&Value::String("foo".into()), &Value::Bool(false));
/// ```
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Same types - this equals the behavior of [PartialEq]
            (Value::String(left), Value::String(right)) => left == right,
            (Value::Numeric(left), Value::Numeric(right)) => left == right,
            (Value::Bool(left), Value::Bool(right)) => left == right,
            (Value::Time(left), Value::Time(right)) => left == right,
            (Value::Duration(left), Value::Duration(right)) => left == right,
            // Implicit conversion: bool vs. numeric where numeric != 0 == true
            (Value::Bool(left), Value::Numeric(right)) => left == &(right != &0f64),
            (Value::Numeric(left), Value::Bool(right)) => &(left != &0f64) == right,
            // Implicit conversion: string vs. numeric where the string must be parseable as `f64`
            (Value::String(left), Value::Numeric(right)) => {
                str::parse::<f64>(left).ok().as_ref() == Some(right)
            }
            (Value::Numeric(left), Value::String(right)) => {
                str::parse::<f64>(right).ok().as_ref() == Some(left)
            }
            // Implicit conversion: string vs. bool where a string is the `display` of the bool
            (Value::String(left), Value::Bool(right)) => left == &format!("{}", right),
            (Value::Bool(left), Value::String(right)) => &format!("{}", left) == right,
            _ => panic!("Unable to compare {:?} against {:?}", self, other),
        }
    }
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
