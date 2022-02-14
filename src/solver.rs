use anyhow::{anyhow, Result};

use crate::{
    compare::{Compareable, Comparison, ComparisonType, Logic},
    resolver::Resolver,
    sequence::{Entity, Sequence},
    value::Value,
    Calculateable, Calculation,
};

fn produce_final_value(
    value: Value,
    calculations: &Vec<Calculation>,
    resolver: &impl Resolver,
) -> Result<Value> {
    let mut init = value;
    for ref item in calculations {
        let (v, a) = match item {
            Calculation::Value(v, op) => (Some(v), op),
            Calculation::Variable(name, op) => (resolver.resolve(name), op),
        };

        let v = v.ok_or_else(|| anyhow!("Unable to resolve variables"))?;

        init = init.calculate(v, *a)?;
    }

    Ok(init)
}

fn resolve_var(comparison: &ComparisonType, resolver: &impl Resolver) -> Result<Value> {
    let (value, calc) = match comparison {
        ComparisonType::Value(ref value, ref calculations) => (Some(value), calculations),
        ComparisonType::Variable(ref lhs, ref calculations) => {
            (resolver.resolve(lhs), calculations)
        }
    };

    let value = value.ok_or_else(|| anyhow!("unable to resolve lhs"))?;
    Ok(produce_final_value(value.clone(), calc, resolver)?)
}

pub fn solve_one(comparison: &Comparison, resolver: &impl Resolver) -> Result<bool> {
    // FIXME: REMOVE
    println!("{:?}", comparison);
    let lhs = resolve_var(&comparison.lhs, resolver)?;
    let rhs = resolve_var(&comparison.rhs, resolver)?;

    Ok(lhs.compare(&rhs, comparison.operator))
}

/// Solve a [Sequence] using the given 'resolver' to a final [bool].
/// In practice, this function throws a sequence of comparisons against a given [Resolver] to evaluate a comparison to true or false.
pub fn solve_tree(sequence: &Sequence, resolver: &impl Resolver) -> Result<bool> {
    let mut result = true;

    for entry in sequence {
        let (child_result, logic) = match entry {
            Entity::Comparison(cmp, logic) => (solve_one(&cmp, resolver)?, logic),
            Entity::Child(seq, logic) => (solve_tree(&seq, resolver)?, logic),
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
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::MapResolver;

    #[test]
    fn test_solve_variable() -> Result<()> {
        use crate::evaluate;

        let mut values = HashMap::new();
        values.insert("a", 1);
        values.insert("b", 2);
        values.insert("foo.bar", 3);
        values.insert("bar_baz", 4);
        let values = MapResolver::from(values);

        assert_eq!(evaluate(r#"a == b - 1"#, &values)?, true);
        assert_eq!(evaluate(r#"b == a + a"#, &values)?, true);
        assert_eq!(evaluate(r#"a < b"#, &values)?, true);
        assert_eq!(evaluate(r#"a + 2 == b + 1"#, &values)?, true);
        assert_eq!(evaluate(r#"a >= b"#, &values)?, false);
        assert_eq!(evaluate(r#"b == a"#, &values)?, false);
        assert_eq!(evaluate(r#"b > a"#, &values)?, true);
        assert_eq!(evaluate(r#"foo.bar < bar_baz"#, &values)?, true);
        assert_eq!(evaluate(r#"bar_baz > b"#, &values)?, true);

        Ok(())
    }

    #[test]
    fn test_solve_duration() -> Result<()> {
        use crate::evaluate;

        let mut values = HashMap::new();
        values.insert("a", chrono::Duration::days(1).to_std()?);
        let values = MapResolver::from(values);

        assert_eq!(evaluate(r#"a < "1 d 1h""#, &values)?, true);
        assert_eq!(evaluate(r#"a > "2h 5min""#, &values)?, true);
        assert_eq!(evaluate(r#"a <= "5h 12min 42sec""#, &values)?, false);

        Ok(())
    }

    #[test]
    fn test_solve_numeric() -> Result<()> {
        use crate::evaluate;

        let mut values = HashMap::new();
        values.insert("a", 1);
        values.insert("b", 2);
        values.insert("c", 3);
        values.insert("d", 4);
        let values = MapResolver::from(values);

        assert_eq!(evaluate("a < 99", &values)?, true);
        assert_eq!(evaluate("a > 2", &values)?, false);
        assert_eq!(evaluate("b <= 2", &values)?, true);
        assert_eq!(evaluate("c >= 3", &values)?, true);
        assert_eq!(evaluate("a != 99 && (b == 2 || c == 99)", &values)?, true);
        assert_eq!(
            evaluate(
                "a == 42 || ((b == 2 || b == 3) && (c == 3 || c == 4))",
                &values
            )?,
            true
        );
        assert_eq!(
            evaluate(
                "a == 4711 || ((b == 42 || b == 2) && (c == 3 && c == 4))",
                &values
            )?,
            false
        );

        assert_eq!(evaluate("((a == 1) && (b == 2))", &values)?, true);
        assert_eq!(evaluate("((((a == 1))))", &values)?, true);
        assert_eq!(evaluate("((((a == 1)))) && b == 2", &values)?, true);
        assert_eq!(evaluate("(a == 1) && b == 3", &values)?, false);
        assert_eq!(evaluate("b == 2 && (a == 1)", &values)?, true);
        assert_eq!(evaluate("(b == 2) && (a == 1)", &values)?, true);

        Ok(())
    }
}
