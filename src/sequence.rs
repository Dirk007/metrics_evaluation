use crate::compare::{ComparisonType, Logic};

/// An entity in a [Sequence] which can be a another [Sequence] [Entity::Child] or a ComparisonType (Value or Variable) [Entity::Comparison],
pub enum Entity {
    Child(Sequence, Option<Logic>),
    Comparison(ComparisonType, Option<Logic>),
}

/// A sequence of [Entity]s which themselfes represents another [Sequence] or a [ComparisonType].
pub type Sequence = Vec<Entity>;
