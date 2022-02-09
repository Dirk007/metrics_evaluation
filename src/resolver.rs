use crate::value::Value;

pub trait Resolver {
    /// Resolves `name` to a [Value] or [None] if there is no value for `name`
    fn resolve(&self, name: impl AsRef<str>) -> Option<&Value>;
}
