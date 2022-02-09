use std::time::Duration;

use chrono::naive::NaiveTime;

use crate::compare::{Compareable, Operator};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    String(String),
    Numeric(f64),
    Bool(bool),
    Time(NaiveTime),
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
