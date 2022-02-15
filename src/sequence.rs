use crate::compare::{Comparison, Logic};

/// An entity in a [Sequence] which can be a another [Sequence] [Entity::Child] or a ComparisonType (Value or Variable) [Entity::Comparison],
pub enum Entity {
    /// Another sequence which is encapsulated by the current sequence
    Child(Sequence, Option<Logic>),
    /// A comparison on the current layer
    Comparison(Comparison, Option<Logic>),
}

/// A sequence of [Entity]s which themselfes represent another [Sequence] or a [Comparison].
pub type Sequence = Vec<Entity>;
