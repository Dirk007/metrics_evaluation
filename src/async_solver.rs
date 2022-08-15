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
    input_value: Value,
    calculations: &Vec<Calculation>,
    resolver: &impl AsyncResolver,
) -> Result<Value> {
    let mut init = input_value;
    for item in calculations {
        let (item_value, item_arithmetic) = match item {
            Calculation::Value(value, arithmetic) => (Some(value), arithmetic),
            Calculation::Variable(name, arithmetic) => (resolver.resolve(name).await, arithmetic),
        };

        let v = item_value.ok_or_else(|| anyhow!("Unable to resolve variables"))?;

        init = init.calculate(v, *item_arithmetic)?;
    }

    Ok(init)
}

async fn resolve_var(comparison: &ComparisonType, resolver: &impl AsyncResolver) -> Result<Value> {
    let (value, calculations) = match comparison {
        ComparisonType::Value(ref value, ref calculations) => (Some(value), calculations),
        ComparisonType::Variable(ref variable_name, ref calculations) => {
            (resolver.resolve(variable_name).await, calculations)
        }
    };

    let value = value.ok_or_else(|| anyhow!("unable to resolve lhs"))?;
    Ok(produce_final_value(value.clone(), calculations, resolver).await?)
}

pub async fn solve_one(comparison: &Comparison, resolver: &impl AsyncResolver) -> Result<bool> {
    let left_variable = resolve_var(&comparison.what, resolver).await?;
    let right_variable = resolve_var(&comparison.against, resolver).await?;

    Ok(left_variable.compare(&right_variable, comparison.operator))
}

#[async_recursion(?Send)]
pub async fn solve_tree(sequence: &Sequence, resolver: &impl AsyncResolver) -> Result<bool> {
    let mut result = true;

    for entry in &sequence.items {
        let (child_result, logic) = match entry {
            Entity::Comparison(comparison, logic) => (solve_one(&comparison, resolver).await?, logic),
            Entity::Child(sequence, logic) => (solve_tree(&sequence, resolver).await?, logic),
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
        evaluate_async("a == 4711 || ((b == 42 || b == 2) && (c == 3 && c == 4))", &values).await?,
        false
    );

    Ok(())
}
