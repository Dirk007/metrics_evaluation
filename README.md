# metrics_evaluation
`metrics_evaluation` is a simple text parser that evaluates a given logic against a [Resolver](src/resolver.rs) (which resolves names to values) resulting in a single `bool`-result. It is possible to perform simple arithmetic operations on the values before the comparison takes place.

It can be used for example to evaluate named IoT-metrics against a given logic to trigger or not to trigger an action if the result of [evaluate](src/lib.rs) results to `true`. The format of the input is equal to the `if`-style of rust (no initial braces needed). Arithmetics are limited to `+` (add), `-` (sub), `*` (mul) and `/` (div) at the moment.

It is possible to evaluate comparisons of variables against fixed values or variables against other variables. Anyway comparisons are limited to always have a variable-name on the left hand at the moment (WIP).

## Usage
Add this crate to your cargo.tml
```
use metrics_evaluation::{parse_tree, solve_tree, Resolver};
```

Implement a [Resolver](src/resolver.rs) that can deliver a [Value](src/value.rs)-reference for the give variable-name-lookup (or none if there is no value for this). For the ease of use and pure laziness there is a [MapResolver](src/mapresolver.rs) wich is a `Resolver` and can be formed from `HashMap<K: AsRef<str>, V: Into<Value>>` (see tests).

Give this `Resolver` to the [evaluate](src/lib.rs) function and call it with a simple text-evaluation as you would do to check if a given value evaluates to true|false in rust. For example: `foo + 2 == 42 && bar < 2 || (baz == true && baz + "30sec" >= "42min")`.

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

The following arithmetic operators are supported but differ in implementation for different [Value](src/value.rs)-Types:
* `+` (add)
* `-` (sub)
* `*` (mul)
* `/` (div)

The following [Value]s can be compared:
* `Value::Numeric` - maps internally to a f64 and has `From`-implementations ranging from `u8` to `f64`
* `Value::String` - a string literal which must be always encapsulated by quotation marks.
* `Value::Time` - maps a [NaiveTime](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html) and the string-representation must match "%H:%M:%S". Must be always encapsulated by quotation marks.
* `Value::Duration` a string that is a [humantime](https://docs.rs/humantime/latest/humantime/) representing a duration. Must be always encapsulated by quotation marks.

The general form of a comparison is `[Name] [Arithmetic] [Comparison-Operator] Value [Arithmetic] [[Logic]...]`.

## Feature-flags
* `async` - additionally have [AsyncResolver] and [AsyncSolver] over [Resolver] and [Solver] for cases a [Resolver] needs async functionality (async database for example). Use `evaluate_async' in this case. `MapResolver` is only available in test-configuration here (as it makes no sense to have such in production).

## Easy example
See [solver](src/solver.rs) tests (end of file) or use it like this:

```rust
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
```

## License
`metrics_evaluation` is distributed under the terms the MIT license.

See [LICENSE](https://github.com/likebike/fasteval/blob/master/LICENSE) for details.


