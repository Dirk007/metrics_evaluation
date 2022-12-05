use std::{collections::HashMap, convert::From};

use crate::{resolver::Resolver, value::Value};

/// Abstraction to use a [HashMap] as a resolver by converting the [HashMap] into a [MapResolver], which implements [Resolver].
/// To make this possible, [From] (AsRef(str), V) is implememnted for each V that is [Into]::[Value].
/// In other words: a HashMap containing a key that is [AsRef]::[str] and a value-type that can be converted to
/// a [Value] ([Value]::[From]::V) can be used as a [Resolver].
pub struct MapResolver(HashMap<String, Value>);

impl<K, V> From<HashMap<K, V>> for MapResolver
where
    K: AsRef<str>,
    Value: From<V>,
{
    fn from(map: HashMap<K, V>) -> Self {
        Self {
            0: map
                .into_iter()
                .map(|(variable_name, value)| (variable_name.as_ref().into(), Value::from(value)))
                .collect(),
        }
    }
}

impl Resolver for MapResolver {
    fn resolve(&self, name: impl AsRef<str>) -> Option<Value> {
        self.0.get(name.as_ref()).map(|value| value.clone())
    }
}

/// This thing only makes sense in tests IMHO
#[cfg(test)]
#[cfg(feature = "async")]
#[async_trait::async_trait]
impl crate::async_resolver::AsyncResolver for MapResolver {
    async fn resolve<'a>(&self, name: impl AsRef<str> + Send + 'a) -> Option<&Value> {
        self.0.get(name.as_ref()).map(|value| value)
    }
}
