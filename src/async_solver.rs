use anyhow::{anyhow, Result};
use async_recursion::async_recursion;

use crate::{
    async_resolver::AsyncResolver,
    compare::{Compareable, Comparison, Logic},
    sequence::{Entity, Sequence},
};

pub async fn solve_one(comparison: &Comparison, resolver: &impl AsyncResolver) -> Result<bool> {
    let value = resolver
        .resolve(&comparison.name)
        .await
        .ok_or_else(|| anyhow!("Unable to resolve '{}'", comparison.name))?;

    Ok(value.compare(&comparison.value, comparison.operator))
}

#[async_recursion(?Send)]
pub async fn solve_tree(sequence: &Sequence, resolver: &impl AsyncResolver) -> Result<bool> {
    let mut result = true;

    for entry in sequence {
        let (child_result, logic) = match entry {
            Entity::Comparison(cmp, logic) => (solve_one(&cmp, resolver).await?, logic),
            Entity::Child(seq, logic) => (solve_tree(&seq, resolver).await?, logic),
        };

        match logic {
            Some(Logic::And) => result &= child_result,
            Some(Logic::Or) => result |= child_result,
            None => result = child_result,
        }
    }

    Ok(result)
}

#[cfg(test)]
#[cfg(feature = "async")]
#[tokio::test]
async fn test_solve_async() -> Result<()> {
    use std::collections::HashMap;

    use crate::{evaluate_async, MapResolver};

    let mut values = HashMap::new();
    values.insert("a", 1);
    values.insert("b", 2);
    values.insert("c", 3);
    values.insert("d", 4);
    let values: MapResolver = values.into();

    assert_eq!(evaluate_async("a < 99", &values).await?, true);
    assert_eq!(evaluate_async("a > 2", &values).await?, false);
    assert_eq!(evaluate_async("b <= 2", &values).await?, true);
    assert_eq!(evaluate_async("c >= 3", &values).await?, true);
    assert_eq!(
        evaluate_async(
            "a == 4711 || ((b == 42 || b == 2) && (c == 3 && c == 4))",
            &values
        )
        .await?,
        false
    );

    Ok(())
}
