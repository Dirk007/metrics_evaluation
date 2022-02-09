use anyhow::{anyhow, Result};

use crate::{
    compare::{Compareable, Comparison, Logic},
    resolver::Resolver,
    sequence::{Entity, Sequence},
};

pub fn solve_one(comparison: &Comparison, resolver: &impl Resolver) -> Result<bool> {
    let value = resolver
        .resolve(&comparison.name)
        .ok_or_else(|| anyhow!("Unable to resolve '{}'", comparison.name))?;

    Ok(value.compare(&comparison.value, comparison.operator))
}

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
