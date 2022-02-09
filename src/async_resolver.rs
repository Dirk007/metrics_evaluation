use async_trait::async_trait;

use crate::value::Value;

#[async_trait]
pub trait AsyncResolver {
    /// Resolves `name` to a [Value] or [None] if there is no value for `name`
    async fn resolve<'a>(&self, name: impl AsRef<str> + Send + 'a) -> Option<&Value>;
}
