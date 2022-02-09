use anyhow::Result;

pub mod compare;
pub mod parser;
pub mod sequence;
pub mod value;

pub use value::Value;

cfg_if::cfg_if! {
    if #[cfg(feature = "async")] {
        pub mod async_resolver;
        pub mod async_solver;
    } else {
        pub mod resolver;
        pub mod solver;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "async")] {
        pub async fn evaluate<'a>(sequence: impl AsRef<str>, resolver: &'a impl async_resolver::AsyncResolver) -> Result<bool> {
            let comparisons = parser::parse_tree(sequence)?;
            async_solver::solve_tree(&comparisons, resolver).await
        }
    } else {
        pub fn evaluate<'a>(sequence: impl AsRef<str>, resolver: &'a impl resolver::Resolver) -> Result<bool> {
            let comparisons = parser::parse_tree(sequence)?;
            solver::solve_tree(&comparisons, resolver)
        }

    }
}
