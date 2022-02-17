//! This crate provides a small-footprint solution to evaluate comparisons, expressed as text, with a dynamically value-lookup.
//!
//! The comparisons can be written like normal rust-code.
//! For example:
//!
//! ```foo + 2 > 2 && bar != 42 || (baz == 47111 && barg * 42 <= 99) && foo >= bar - 5```.
//! 
//! The only limitation at the moment is that you currently have to use a variable-name on the left-hand (WIP). Left-Hand values are not supported yet.
//!
//! Comparisons can be made against any [Value]-Type implemented:
//! - [Value::String] encapsulated in quotation marks
//! - [Value::Numeric] (internally [f64] but everything from u8 to f64 will be converted Into it automatically)
//! - [Value::Bool] which is simply a [bool]
//! - [Value::Time] (internally [chrono::NaiveTime]), encapsulated in quotation marks and expressed in form of "%H:%M:%S" as of NaiveTime::parse_from_str().
//! - [Value::Duration] as [chrono::Duration] encapsulated in quotation marks and represented in [humantime::Duration] (see [humantime::parse_duration] for formatting possibilities) for ease of use
//!
//! Value-Lookup is made through a given [Resolver]-trait internally so you are open to use what ever you like in the background to resolve variable-names to their value-representation.
//!
//! If you want to use an async resolver (see AsyncResolver), you have to enable the `async` feature.
//!
//! For laziness there is a [MapResolver] which implements [Resolver] and can be made [From] any [std::collections::HashMap] that contain a Key which is [AsRef]\<str> and values which can be made [Value]::[From].
//!
//! For the ease of use, an `crate::evaluate` function is implemented which just takes a string and compares using a given resolver.
//!
//! To have a more performant usage of this crate, use [crate::parse_tree] which produces a pre-parsed [Sequence] once.
//! This [Sequence] can then be used in subsequent calls to [crate::solve_tree] to evalaute the [Sequence] with current variable-values over and over again.

use anyhow::Result;

/// Compute arithmetics on [Value]s
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

pub use calculate::{Arithmetic, Calculateable, Calculation};
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

/// A serde deserializer for [Sequence]
#[cfg(feature = "serde_de")]
pub mod serde_de;

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
