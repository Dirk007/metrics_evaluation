//! This crate provides a solution to evaluate comparison-texts against given metric-values - for example to evaluate
//! IoT-Triggers against a collection of comparisons ([Sequence] here).
//!
//! The comparisons can be written like normal rust-code ("foo > 2 && bar != 42 || (baz == 47111 && barg <= 99)"). Comparisons
//! can be made against String, Numeric (internally f64), Time (internally NaiveTime) and Duration (represented in humantime).
//!
//! Value-Lookup is made through a given [Resolver] internally so you are open to use what ever you like in the background.

use anyhow::Result;

/// Compare [Value] against [Value]
pub mod compare;
/// Helper-Object to use [HashMap] as [Resolver]
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

pub use mapresolver::MapResolver;
pub use resolver::Resolver;
pub use value::Value;

/// Evaluate `sequence` with the given [Resolver] resolver to a final bool-result
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
