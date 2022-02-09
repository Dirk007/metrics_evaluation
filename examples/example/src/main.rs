use std::collections::HashMap;

use anyhow::Result;
use metrics_evaluation::{evaluate, Value};

fn main() -> Result<()> {
    let mut values = HashMap::new();
    values.insert("temp", Value::Numeric(10.0));
    values.insert("humidity", Value::Numeric(80.0));
    values.insert(
        "working",
        Value::Duration(chrono::Duration::hours(2).to_std().unwrap()),
    );

    assert_eq!(
        evaluate(
            r#"(temp > 1 && humidity <= 80) || working > "1h 5min""#,
            &values
        )?,
        true
    );

    Ok(())
}
