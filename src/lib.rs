//! This crate provides a small-foorprint solution to evaluate comparison-texts against given metric-values - for example to evaluate
//! IoT-Triggers against a collection of comparisons ([Sequence] here).
//!
//! The comparisons can be written like normal rust-code ("foo > 2 && bar != 42 || (baz == 47111 && barg <= 99) && foo >= bar").
//! Comparisons can be made against [Value::String], [Value::Numeric] (internally [f64]), [Value::Time] (internally [chrono::NaiveTime]) and [Value::Duration] as [chrono::Duration]
//! (represented in [humantime]) and other variable-names.
//!
//! Value-Lookup is made through a given [Resolver]-trait internally so you are open to use what ever you like in the background.
//! For the ease of use, an `crate::evaluate` function is implemented which just takes a string and compares using a given resolver.
//! To have a more performant usage of this crate, use [crate::parse_tree] once with a call to [crate::solve_tree] periodically or when needed.

use anyhow::Result;

pub mod calculate;
/// Compare [Value] against [Value]
pub mod compare;
/// Helper-Object to use [std::collections::HashMap] as [Resolver]
pub mod mapresolver;
/// Parser to generate [Sequence] from a given text
pub mod parser;
/// Resolves name to [Value]
pub mod resolver;
/// Sequence of comparisons
pub mod sequence;
/// Solves [Sequence]
pub mod solver;
/// A generic value
pub mod value;

pub use calculate::{Arithmetic, Calculateable};
pub use mapresolver::MapResolver;
pub use parser::parse_tree;
pub use resolver::Resolver;
pub use sequence::Sequence;
pub use solver::solve_tree;
pub use value::Value;

/// Evaluate string-`sequence` with the given [Resolver] resolver to a final bool-result.
/// This always parses the `sequence`-string, generates a [Sequence] and evaluats it using the given [Resolver].
/// Use this if the input-sequence is changing on the same logic. To have a better performing solution where
/// input-sequences do not change and where you just want to check a given logic against changing metrics, save the
/// output of [parse_tree] and throw it towards a value-changing [Resolver] in a [solve_tree] when needed.
pub fn evaluate<'a>(
    sequence: impl AsRef<str>,
    resolver: &'a impl resolver::Resolver,
) -> Result<bool> {
    let comparisons = parser::parse_tree(sequence)?;
    solver::solve_tree(&comparisons, resolver)
}

#[cfg(feature = "async")]
pub mod async_resolver;
#[cfg(feature = "async")]
pub mod async_solver;
#[cfg(feature = "async")]
pub use async_resolver::AsyncResolver;

#[cfg(feature = "async")]
/// Async-version of 'evaluate'
pub async fn evaluate_async<'a>(
    sequence: impl AsRef<str>,
    resolver: &'a impl async_resolver::AsyncResolver,
) -> Result<bool> {
    let comparisons = parser::parse_tree(sequence)?;
    async_solver::solve_tree(&comparisons, resolver).await
}
