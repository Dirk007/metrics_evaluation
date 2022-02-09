use anyhow::Result;

pub mod compare;
pub mod mapresolver;
pub mod parser;
pub mod sequence;
pub mod value;

pub mod resolver;
pub mod solver;

pub use mapresolver::MapResolver;
pub use resolver::Resolver;
pub use value::Value;

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
pub async fn evaluate_async<'a>(
    sequence: impl AsRef<str>,
    resolver: &'a impl async_resolver::AsyncResolver,
) -> Result<bool> {
    let comparisons = parser::parse_tree(sequence)?;
    async_solver::solve_tree(&comparisons, resolver).await
}
