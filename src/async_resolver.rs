#[cfg(feature = "hashmap")]
use std::collections::HashMap;

use async_trait::async_trait;

use crate::value::Value;

#[async_trait]
pub trait AsyncResolver {
    /// Resolves `name` to a [Value] or [None] if there is no value for `name`
    async fn resolve<'a>(&self, name: impl AsRef<str> + Send + 'a) -> Option<Value>;
}

/// Helper to lazily use a [HashMap] as a resolver
#[cfg(feature = "hashmap")]
#[async_trait]
impl<V> AsyncResolver for HashMap<&str, V>
where
    V: Clone + Send + Sync,
    Value: From<V>,
{
    async fn resolve<'a>(&self, name: impl AsRef<str> + Send + 'a) -> Option<Value> {
        self.get(name.as_ref())
            .map(|value| Value::from(value.clone()))
    }
}
