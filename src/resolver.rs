use crate::value::Value;

/// Resolves `name` to [Value] or [None] if there is no value for that given variable-`name`.
/// Used for solving comparisons in [crate::solver::solve_tree] and [crate::evaluate].
pub trait Resolver {
    /// Resolves `name` to a [Value] or [None] if there is no value for `name`
    fn resolve<T: AsRef<str> + std::fmt::Debug>(&self, name: &T) -> Option<Value>;
}
