use anyhow::{anyhow, Result};
use async_recursion::async_recursion;

use crate::{
    async_resolver::AsyncResolver,
    calculate::{Calculateable, Calculation},
    compare::{Compareable, Comparison, ComparisonType, Logic},
    sequence::{Entity, Sequence},
    value::Value,
};

async fn produce_final_value(
    value: Value,
    calculations: &Vec<Calculation>,
    resolver: &impl AsyncResolver,
) -> Result<Value> {
    let mut init = value;
    for ref item in calculations {
        let (v, a) = match item {
            Calculation::Value(v, op) => (Some(v), op),
            Calculation::Variable(name, op) => (resolver.resolve(name).await, op),
        };

        let v = v.ok_or_else(|| anyhow!("Unable to resolve variables"))?;

        init = init.calculate(v, *a)?;
    }

    Ok(init)
}

async fn resolve_var(comparison: &ComparisonType, resolver: &impl AsyncResolver) -> Result<Value> {
    let (value, calc) = match comparison {
        ComparisonType::Value(ref value, ref calculations) => (Some(value), calculations),
        ComparisonType::Variable(ref lhs, ref calculations) => {
            (resolver.resolve(lhs).await, calculations)
        }
    };

    let value = value.ok_or_else(|| anyhow!("unable to resolve lhs"))?;
    Ok(produce_final_value(value.clone(), calc, resolver).await?)
}

pub async fn solve_one(comparison: &Comparison, resolver: &impl AsyncResolver) -> Result<bool> {
    let lhs = resolve_var(&comparison.lhs, resolver).await?;
    let rhs = resolve_var(&comparison.rhs, resolver).await?;

    Ok(lhs.compare(&rhs, comparison.operator))
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
