use crate::compare::{Comparison, Logic};

/// An entity in a [Sequence]
pub enum Entity {
    Child(Sequence, Option<Logic>),
    Comparison(Comparison, Option<Logic>),
}

/// A sequence of [Entity]s
pub type Sequence = Vec<Entity>;
