use std::collections::HashMap;

use anyhow::Result;
use metrics_evaluation::{evaluate, MapResolver, Value};

fn main() -> Result<()> {
    let mut values = HashMap::new();
    values.insert("room1.temp", Value::Numeric(20.0));
    values.insert("room2.temp", Value::Numeric(22.0));
    values.insert("room1.humidity", Value::Numeric(80.0));
    values.insert(
        "working",
        Value::Duration(chrono::Duration::hours(2).to_std().unwrap()),
    );
    let values: MapResolver = values.into();

    assert_eq!(
        evaluate(
            r#"(room1.temp > 1 && room1.humidity <= 80) || working > "1h 5min""#,
            &values
        )?,
        true
    );

    assert_eq!(
        evaluate(r#"room2.temp > room1.temp && room2.temp < 25"#, &values)?,
        true
    );

    Ok(())
}
