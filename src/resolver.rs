#[cfg(feature = "hashmap")]
use std::collections::HashMap;

use crate::value::Value;

pub trait Resolver {
    /// Resolves `name` to a [Value] or [None] if there is no value for `name`
    fn resolve(&self, name: impl AsRef<str>) -> Option<Value>;
}

#[cfg(feature = "hashmap")]
/// Helper to lazily use a [HashMap] as a resolver
impl<V> Resolver for HashMap<&str, V>
where
    V: Clone,
    Value: From<V>,
{
    fn resolve(&self, name: impl AsRef<str>) -> Option<Value> {
        self.get(name.as_ref())
            .map(|value| Value::from(value.clone()))
    }
}
