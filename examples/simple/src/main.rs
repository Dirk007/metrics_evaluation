use std::collections::HashMap;

use anyhow::Result;
use metrics_evaluation::{evaluate, MapResolver, Value};

fn main() -> Result<()> {
    let mut values = HashMap::new();
    values.insert("room1.temp", Value::Numeric(20.0));
    values.insert("room2.temp", Value::Numeric(22.0));
    values.insert("room1.humidity", Value::Numeric(80.0));
    values.insert(
        "worktime",
        Value::Duration(*"1h 5min 42s".parse::<humantime::Duration>().unwrap()),
    );
    let values: MapResolver = values.into();

    assert_eq!(
        evaluate(
            r#"(room1.temp > 1 || room1.humidity <= 80) && worktime > "1h 5min""#,
            &values
        )?,
        true
    );

    assert_eq!(evaluate(r#"worktime + "30m" > "1h""#, &values)?, true);

    assert_eq!(
        evaluate(
            r#"room1.temp + 2 == 22 && room2.temp > 30 || (room1.humidity >= 80 && worktime + "30sec" >= "42min")"#,
            &values
        )?,
        true
    );

    assert_eq!(
        evaluate(r#"room2.temp > room1.temp && room2.temp < 25"#, &values)?,
        true
    );

    assert_eq!(evaluate(r#"room1.temp + 2 == room2.temp"#, &values)?, true);

    assert_eq!(
        evaluate(r#"room2.temp - 1  < room1.temp * 2"#, &values)?,
        true
    );

    Ok(())
}
