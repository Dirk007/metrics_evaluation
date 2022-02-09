# metrics_evaluation
`metrics_evaluation` is a simple text parser that evaluates a given logic against a metrics-resolver resulting in a single `bool`-result.
It can be used for example to evaluate named IoT-metrics against a given logic to trigger or not to trigger an action if the result of [evaluate](src/lib.rs) results to `true`.
The format of the input is equal to the `if`-style of rust (no initial braces needed).

Give this `Resolver` to the [evaluate](src/lib.rs) function and call it with a simple text-evaluation as you would do to check if a given value evaluates to true|false in rust. For example: `foo == 42 && bar < 2 || (baz == true)`.

## Usage
TBD: Add this crate to your cargo.tml

Implement a [Resolver](src/resolver.rs) that can deliver a [Value](src/value.rs)-reference for the give lookup (or none if there is no value for this). For the ease of use and pure laziness there is a type `MapResolver` wich is a `Resolver`and can be formed fropm `HashMap<K: AsRef<str>, V: Into<Value>>` (see tests).

The following operators are supported and behave like in rust:
* `>`
* `<`
* `>=`
* `<=`
* `==`
* `!=`

The following logics are supported and behave like in rust:
* `and`, `&&`
* `or`, `||`

The following [Value]s can be compared:
* `Value::Numeric` - maps internally to a f64 and has `From`-implementations ranging from `u8` to `f64`
* `Value::String` - a string literal
* `Value::Time` - maps a [NaiveTime](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html) and the string-representation must match one of the NaiveTime's ::from(&str) implementations.
* `Value::Duration` a string that is a [humantime](https://docs.rs/humantime/latest/humantime/) representing a duration

Use it in the form [Name] [Operator] Value [[Logic]...]

## Feature-flags
* `async` - additionally have [AsyncResolver] and [AsyncSolver] over [Resolver] and [Solver] for cases a [Resolver] needs async functionality (async database for example). Use `evaluate_async' in this case.

## Easy example
See [lib.rs] tests or use it like this
```rust
use std::collections::HashMap;

use anyhow::Result;
use metrics_evaluation::{evaluate, MapResolver, Value};

fn main() -> Result<()> {
    let mut values = HashMap::new();
    values.insert("temp", Value::Numeric(10.0));
    values.insert("humidity", Value::Numeric(80.0));
    values.insert(
        "working",
        Value::Duration(chrono::Duration::hours(2).to_std().unwrap()),
    );
    let values: MapResolver = values.into();

    assert_eq!(
        evaluate(
            r#"(temp > 1 && humidity <= 80) || working > "1h 5min""#,
            &values
        )?,
        true
    );

    Ok(())
}
```

## License
`metrics_evaluation` is distributed under the terms the MIT license.

See [LICENSE](https://github.com/likebike/fasteval/blob/master/LICENSE) for details.


