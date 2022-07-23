use anyhow::{anyhow, Result};

use crate::{
    compare::{Compareable, Comparison, ComparisonType, Logic},
    resolver::Resolver,
    sequence::{Entity, Sequence},
    value::Value,
    Calculateable, Calculation,
};

fn produce_final_value(input_value: Value, calculations: &Vec<Calculation>, resolver: &impl Resolver) -> Result<Value> {
    let mut current = input_value;
    for item in calculations {
        let (item_value, item_arithmetic) = match item {
            Calculation::Value(value, artihmetic) => (Some(value), artihmetic),
            Calculation::Variable(name, artihmetic) => (resolver.resolve(name), artihmetic),
        };

        let value = item_value.ok_or_else(|| anyhow!("Unable to resolve variable {:?}", item))?;

        current = current.calculate(value, *item_arithmetic)?;
    }

    Ok(current)
}

fn resolve_var(comparison: &ComparisonType, resolver: &impl Resolver) -> Result<Value> {
    let (item_value, item_calculations) = match comparison {
        ComparisonType::Value(ref value, ref value_calculations) => (Some(value), value_calculations),
        ComparisonType::Variable(ref variable_name, ref variable_calculations) => {
            (resolver.resolve(variable_name), variable_calculations)
        }
    };

    let value = item_value.ok_or_else(|| anyhow!("unable to resolve lhs in {:?}", comparison))?;
    Ok(produce_final_value(value.clone(), item_calculations, resolver)?)
}

pub fn solve_one(comparison: &Comparison, resolver: &impl Resolver) -> Result<bool> {
    let left_value = resolve_var(&comparison.what, resolver)?;
    let right_value = resolve_var(&comparison.against, resolver)?;

    let result = left_value.compare(&right_value, comparison.operator);

    Ok(result)
}

/// Solve a [Sequence] using the given 'resolver' to a final [bool].
/// In practice, this function throws a sequence of comparisons against a given [Resolver] to evaluate a comparison to true or false.
pub fn solve_tree(sequence: &Sequence, resolver: &impl Resolver) -> Result<bool> {
    let mut result = true;

    for entry in &sequence.items {
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
    fn test_complex() -> Result<()> {
        use chrono::naive::NaiveTime;

        use crate::evaluate;

        // Assertion is: Window is open between 09:00 and 22:00 AND
        //   - the window is open for more than 10 minutes AND the room temperature has droppped below 20 degrees
        //   OR
        //   - the window is open for more than 30 minutes
        //   OR
        //   - the temperature has dropped below 10 degreesd
        const QUERY: &str = r#"window.contact.value == false && compute.time.now > "09:00:00" && compute.time.now < "22:00:00" && 
                    ((window.contact.since > "10min" && room.temperature.value < 20) || window.contact.since > "30min" || room.temperature.value < 10)"#;

        struct Test {
            pub description: &'static str,
            pub open: bool,
            pub open_since_minutes: u64,
            pub room_tempereature: f64,
            pub current_time: NaiveTime,
            pub expected_to_trigger: bool,
        }

        let tests = [Test {
            description: "Inside timeframe: window open, but warm enough -> do not trigger",
            open: true,
            open_since_minutes: 10,
            room_tempereature: 21.0,
            current_time: NaiveTime::from_hms(21, 1, 0),
            expected_to_trigger: false,
        },
        Test {
            description:
                "Inside timeframe: window open, warm enough, but open for too long -> trigger",
            open: true,
            open_since_minutes: 60,
            room_tempereature: 21.0,
            current_time: NaiveTime::from_hms(21, 1, 0),
            expected_to_trigger: true,
        },
        Test {
            description: "Inside timeframe: window open, open short enough but too cold -> trigger",
            open: true,
            open_since_minutes: 10,
            room_tempereature: 9.0,
            current_time: NaiveTime::from_hms(21, 1, 0),
            expected_to_trigger: true,
        },
        Test {
            description:
                "OUTSIDE timeframe: window open, open short enough but too cold -> do not trigger",
            open: true,
            open_since_minutes: 10,
            room_tempereature: 9.0,
            current_time: NaiveTime::from_hms(23, 1, 0),
            expected_to_trigger: false,
        },
        Test {
            description:
                "OUTSIDE timeframe: window open, open too long and too cold -> do not trigger",
            open: true,
            open_since_minutes: 120,
            room_tempereature: 1.0,
            current_time: NaiveTime::from_hms(01, 1, 0),
            expected_to_trigger: false,
        },
        Test {
            description:
                "INSIDE timeframe: window open, open too long and too cold -> trigger (same as last one but inside timeframe)",
            open: true,
            open_since_minutes: 120,
            room_tempereature: 1.0,
            current_time: NaiveTime::from_hms(10, 1, 0),
            expected_to_trigger: true,
        },
        Test {
            description:
                "INSIDE timeframe: window CLOSED, open too long and too cold -> do not trigger",
            open: false,
            open_since_minutes: 120,
            room_tempereature: 1.0,
            current_time: NaiveTime::from_hms(10, 1, 0),
            expected_to_trigger: false,
        },
        Test {
            description:
                "OUTSIDE timeframe: window open, open short enough and warm enough -> do not trigger",
            open: true,
            open_since_minutes: 10,
            room_tempereature: 30.0,
            current_time: NaiveTime::from_hms(10, 1, 0),
            expected_to_trigger: false,
        }];

        for test in tests {
            println!("Complex test: {}", test.description);
            let mut values = HashMap::new();
            values.insert("window.contact.value", Value::Bool(!test.open));
            values.insert(
                "window.contact.since",
                Value::Duration(core::time::Duration::from_secs(test.open_since_minutes * 60)),
            );
            values.insert("room.temperature.value", Value::Numeric(test.room_tempereature));
            values.insert("compute.time.now", Value::Time(test.current_time));
            let values = MapResolver::from(values);

            assert_eq!(evaluate(QUERY, &values)?, test.expected_to_trigger);
        }

        Ok(())
    }

    #[test]
    fn test_solve_time() -> Result<()> {
        use chrono::naive::NaiveTime;

        use crate::evaluate;

        let mut values = HashMap::new();
        values.insert("start", Value::Time(NaiveTime::parse_from_str("05:00:00", "%H:%M:%S")?));
        values.insert("now", Value::Time(NaiveTime::parse_from_str("15:00:00", "%H:%M:%S")?));
        values.insert("end", Value::Time(NaiveTime::parse_from_str("22:00:00", "%H:%M:%S")?));

        let values = MapResolver::from(values);

        assert_eq!(evaluate(r#"start <= "15:00:00""#, &values)?, true);
        assert_eq!(evaluate(r#"end >= "15:00:00""#, &values)?, true);
        assert_eq!(evaluate(r#"now >= "22:00:00""#, &values)?, false);
        assert_eq!(evaluate(r#"now <= "05:00:00""#, &values)?, false);
        assert_eq!(evaluate(r#"now >= "22:00:00" || now <= "05:00:00""#, &values)?, false);
        assert_eq!(evaluate(r#"now >= start && now <= end"#, &values)?, true);

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
            evaluate("a == 42 || ((b == 2 || b == 3) && (c == 3 || c == 4))", &values)?,
            true
        );
        assert_eq!(
            evaluate("a == 4711 || ((b == 42 || b == 2) && (c == 3 && c == 4))", &values)?,
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
