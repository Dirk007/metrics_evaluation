use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor};

use crate::{parser::parse_tree, Sequence};

pub struct SequenceVisitor;

impl<'de> Visitor<'de> for SequenceVisitor {
    type Value = Sequence;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a (nested) comparison")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_tree(value).map_err(|e| E::custom(format!("Unable to parse '{}': {:?}", value, e)))
    }
}

impl<'de> Deserialize<'de> for Sequence {
    fn deserialize<D>(deserializer: D) -> Result<Sequence, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(SequenceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::compare::{Comparison, Logic, Operator};
    use crate::sequence::{Entity, Sequence};
    use crate::value::Value;

    #[test]
    fn test_deserialize() -> anyhow::Result<()> {
        use serde::de::value::{Error as ValueError, StrDeserializer};
        use serde::de::IntoDeserializer;

        let input = "a > 1 && b < 2";

        let deserializer: StrDeserializer<ValueError> = input.into_deserializer();
        let seq = Sequence::deserialize(deserializer).expect("Unable to deserialize");
        assert_eq!(
            seq.items[0],
            Entity::Comparison(
                Comparison::from(("a", Operator::Greater, Value::Numeric(1.0))),
                None
            ),
        );

        assert_eq!(
            seq.items[1],
            Entity::Comparison(
                Comparison::from(("b", Operator::Less, Value::Numeric(2.0))),
                Some(Logic::And)
            ),
        );

        Ok(())
    }
}
