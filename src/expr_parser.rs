use anyhow::{anyhow, Result};
use chrono::naive::NaiveTime;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::{
        complete::{alpha1, alphanumeric1, char},
        streaming::multispace0,
    },
    combinator::{recognize, value},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use parse_hyperlinks::take_until_unbalanced;

use crate::{
    calculate::Arithmetic,
    compare::{Comparison, ComparisonType, Logic, Operator},
    sequence::{Entity, Sequence},
    value::Value,
    Calculation,
};

const TRUE: &str = "true";
const FALSE: &str = "false";

fn match_optional_logic(input: &str) -> IResult<&str, Vec<&str>> {
    // TODO: isnt there something like `one0`?
    let (rest, m) = many0(alt((
        trim(tag("or")),
        trim(tag("and")),
        trim(tag("||")),
        trim(tag("&&")),
    )))(input)?;
    Ok((rest, m))
}

fn match_optional_calculation(input: &str) -> IResult<&str, Vec<&str>> {
    // TODO: isnt there something like `one0`?
    let (rest, m) = many0(alt((trim(tag("+")), trim(tag("-")), trim(tag("*")), trim(tag("/")))))(input)?;
    Ok((rest, m))
}

fn match_identifier(input: &str) -> IResult<&str, &str> {
    let (rest, m) = recognize(pair(
        alt((alpha1, tag("."), tag("_"))),
        many0(alt((alphanumeric1, tag("."), tag("_")))),
    ))(input)?;
    Ok((rest, m))
}

fn match_value(input: &str) -> IResult<&str, Value> {
    let (rest, value) = recognize(many1(alphanumeric1))(input)?;

    let value = match value {
        TRUE => true.into(),
        FALSE => false.into(),
        _ => str::parse::<f64>(value)
            .map_err(|_| nom::Err::Incomplete(nom::Needed::Unknown))?
            .into(),
    };

    Ok((rest, value))
}

fn match_string_literal(input: &str) -> IResult<&str, &str> {
    let (rest, m) = recognize(delimited(
        alt((char('"'), char('\''))),
        many1(is_not("\"")),
        alt((char('"'), char('\''))),
    ))(input)?;
    Ok((rest, &m[1..m.len() - 1]))
}

fn match_string_type(input: &str) -> IResult<&str, Value> {
    let (rest, value) = match_string_literal(input)?;

    let value = humantime::parse_duration(value)
        .map(|duration| Value::from(duration))
        .or_else(|_| NaiveTime::parse_from_str(&value, "%H:%M:%S").map(|time| Value::from(time)))
        .unwrap_or_else(|_| Value::from(value));

    Ok((rest, value))
}

fn match_value_type(input: &str) -> IResult<&str, Value> {
    let (rest, value) = alt((match_value, match_string_type))(input)?;
    Ok((rest, value))
}

/// Remove whitespaces around
fn trim<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// Remove trailing whitespace
fn trim_front<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
}

fn match_compare_op(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Equal, trim(tag("=="))),
        value(Operator::NotEqual, trim(tag("!="))),
        value(Operator::GreaterEqual, trim(tag(">="))),
        value(Operator::LessEqual, trim(tag("<="))),
        value(Operator::Greater, trim(tag(">"))),
        value(Operator::Less, trim(tag("<"))),
    ))(input)
}

fn match_calc_op(input: &str) -> IResult<&str, Arithmetic> {
    alt((
        value(Arithmetic::Add, trim(tag("+"))),
        value(Arithmetic::Sub, trim(tag("-"))),
        value(Arithmetic::Mul, trim(tag("*"))),
        value(Arithmetic::Div, trim(tag("/"))),
    ))(input)
}

fn match_varval_comparison(input: &str) -> IResult<&str, Comparison> {
    let (rest, (identifier, calcs_lhs, op, value, calcs_rhs)) = tuple((
        match_identifier,
        match_calculations,
        match_compare_op,
        match_value_type,
        match_calculations,
    ))(input)?;

    Ok((
        rest,
        Comparison {
            what: ComparisonType::Variable(identifier.into(), calcs_lhs),
            operator: op,
            against: ComparisonType::Value(value, calcs_rhs),
        },
    ))
}

fn match_varvar_comparison(input: &str) -> IResult<&str, Comparison> {
    let (rest, (lhs, calcs_lhs, op, rhs, calcs_rhs)) = tuple((
        match_identifier,
        match_calculations,
        match_compare_op,
        match_identifier,
        match_calculations,
    ))(input)?;

    Ok((
        rest,
        Comparison {
            what: ComparisonType::Variable(lhs.into(), calcs_lhs),
            operator: op,
            against: ComparisonType::Variable(rhs.into(), calcs_rhs),
        },
    ))
}

fn match_value_calculation(input: &str) -> IResult<&str, Calculation> {
    let (rest, (arithmetic, value)) = tuple((match_calc_op, match_value_type))(input)?;

    let calculation = Calculation::Value(value, arithmetic);

    Ok((rest, calculation))
}

fn match_variable_calculation(input: &str) -> IResult<&str, Calculation> {
    let (rest, (arithmetic, name)) = tuple((match_calc_op, match_identifier))(input)?;

    let calculation = Calculation::Variable(name.into(), arithmetic);

    Ok((rest, calculation))
}

fn match_calculations(input: &str) -> IResult<&str, Vec<Calculation>> {
    let mut rest = input;
    let mut result = Vec::new();
    // Try to acquire any appended arithmetic to the comparison (e.G. a == foo + 2)
    while !rest.is_empty() {
        let (_, arithmetic) = match_optional_calculation(rest)?;
        if arithmetic.is_empty() {
            break;
        }
        let (new_rest, calculation) = match_value_calculation(rest).or_else(|_| match_variable_calculation(rest))?;

        result.push(calculation);
        rest = new_rest.trim();
    }

    Ok((rest, result))
}

fn match_comparison(input: &str) -> IResult<&str, Comparison> {
    // This fails: alt((match_value_comparison, match_variable_comparison))(input)
    let (rest, comparison) = match_varval_comparison(input).or_else(|_| match_varvar_comparison(input))?;

    Ok((rest, comparison))
}

/// Matches one underlying block with optional logic.
///
/// ```
/// use metrics_evaluation::{
///     compare::{Logic, Operator},
///     expr_parser::match_block,
/// };
///
/// let (rest, (block, logic)) = match_block(
///     "(hey > 2 and (foo < 1) or ((bar == 2 and foo == 3) or (baz != 42)) or bam == 99)",
/// )
/// .unwrap();
/// assert_eq!(
///     block,
///     "hey > 2 and (foo < 1) or ((bar == 2 and foo == 3) or (baz != 42)) or bam == 99"
/// );
/// assert_eq!(rest, "");
/// assert_eq!(logic, None);
/// // Either<Comparison, Block> is Comparison
/// let (rest, (block, logic)) =
///     match_block("and (foo < 1) or ((bar == 2 and foo == 3) or (baz != 42)) or bam == 99")
///         .unwrap();
/// assert_eq!(block, "foo < 1");
/// assert_eq!(logic, Some(Logic::And));
/// assert_eq!(
///     rest,
///     " or ((bar == 2 and foo == 3) or (baz != 42)) or bam == 99"
/// );
/// let (rest, (block, logic)) = match_block(rest).unwrap();
/// assert_eq!(block, "(bar == 2 and foo == 3) or (baz != 42)");
/// assert_eq!(logic, Some(Logic::Or));
/// assert_eq!(rest, " or bam == 99"); //< todo after that block is solved
/// let (rest, (block, logic)) = match_block(block).unwrap();
/// assert_eq!(block, "bar == 2 and foo == 3");
/// assert_eq!(logic, None);
/// assert_eq!(rest, " or (baz != 42)");
///
/// // Test if whitespace on last blocks works
/// let (rest, (block, logic)) = match_block("(hello > 1 && (foo == 2 || foo == 3) )").unwrap();
/// assert_eq!(block, "hello > 1 && (foo == 2 || foo == 3) ");
/// assert_eq!(logic, None);
/// assert_eq!(rest, "");
/// let (rest, (block, logic)) = match_block("&& (foo == 2 || foo == 3) ").unwrap();
/// assert_eq!(block, "foo == 2 || foo == 3");
/// ```
pub fn match_block(input: &str) -> IResult<&str, (&str, Option<Logic>)> {
    let (rest, (logics, block)) = tuple((
        match_optional_logic,
        delimited(trim(tag("(")), take_until_unbalanced('(', ')'), trim_front(tag(")"))),
    ))(input)?;

    let logic = decode_logic(logics);

    Ok((rest, (block, logic)))
}

fn decode_logic(logics: Vec<&str>) -> Option<Logic> {
    (!logics.is_empty())
        .then(|| match logics[0] {
            "and" => Some(Logic::And),
            "or" => Some(Logic::Or),
            "&&" => Some(Logic::And),
            "||" => Some(Logic::Or),
            _ => None,
        })
        .flatten()
}

/// Matches one comparison with optional logic.
/// ```
/// use metrics_evaluation::{
///     calculate::{Arithmetic, Calculation},
///     compare::{Comparison, Logic, Operator},
///     expr_parser::match_comparisons,
///     value::Value,
/// };
///
/// let (rest, (cmp, logic)) = match_comparisons("hello > 1 + 2 - foo").unwrap();
/// let mut expected = Comparison::from(("hello", Operator::Greater, Value::from(1)));
/// expected
///     .against
///     .with_calculation(Calculation::Value(2.into(), Arithmetic::Add));
/// expected
///     .against
///     .with_calculation(Calculation::Variable("foo".into(), Arithmetic::Sub));
/// assert_eq!(cmp, expected);
/// assert_eq!(logic, None);
///
/// let (rest, (cmp, logic)) = match_comparisons("hello > 1 and foo < 2").unwrap();
/// assert_eq!(
///     cmp,
///     Comparison::from(("hello", Operator::Greater, Value::from(1)))
/// );
/// assert_eq!(rest, " and foo < 2");
/// assert_eq!(logic, None);
///
/// let (rest, (cmp, logic)) = match_comparisons(rest).unwrap();
/// assert_eq!(
///     cmp,
///     Comparison::from(("foo", Operator::Less, Value::from(2)))
/// );
/// assert_eq!(logic, Some(Logic::And));
///
/// let (rest, (cmp, logic)) = match_comparisons(r#"foo == "bar""#).unwrap();
/// assert_eq!(
///     cmp,
///     Comparison::from(("foo", Operator::Equal, Value::from("bar")))
/// );
/// assert_eq!(logic, None);
///
/// let (rest, (cmp, logic)) = match_comparisons("hello > foo or bar == foo").unwrap();
/// assert_eq!(cmp, Comparison::from(("hello", Operator::Greater, "foo")));
/// assert_eq!(rest, " or bar == foo");
/// assert_eq!(logic, None);
///
/// let (rest, (cmp, logic)) = match_comparisons(rest).unwrap();
/// assert_eq!(cmp, Comparison::from(("bar", Operator::Equal, "foo")));
/// assert_eq!(logic, Some(Logic::Or));
/// ```
pub fn match_comparisons(input: &str) -> IResult<&str, (Comparison, Option<Logic>)> {
    let (rest, (logics, comparison)) = tuple((match_optional_logic, match_comparison))(input)?;

    let logic = decode_logic(logics);

    Ok((rest, (comparison, logic)))
}

/// Parse `input` recursively and produce a [Sequence] from all children.
/// This [Sequence] can be thrown against a [crate::solver::solve_tree] using a [crate::resolver::Resolver] to solve the `input`.
/// Use [crate::evaluate] to have an already implemented combination.
/// ```
/// use metrics_evaluation::*;
///
/// // Test for weird whitespaces
/// assert!(parse_tree("hello > 1 && ( foo == 2 || foo == 3)").is_ok());
/// assert!(parse_tree("hello > 1 && (foo == 2 || foo == 3 )").is_ok());
/// assert!(parse_tree("hello > 1 && (foo == 2 || foo == 3 ) ").is_ok());
/// ```
pub fn parse_tree(input: impl AsRef<str>) -> Result<Sequence> {
    let mut rest: &str = input.as_ref();
    let mut results: Sequence = Sequence::default();

    while !rest.is_empty() {
        let (block, comparison) = (match_block(rest), match_comparisons(rest));
        match (block, comparison) {
            (Ok((new_rest, (block, logic))), Err(_)) => {
                rest = new_rest.trim();
                results.items.push(Entity::Child(parse_tree(block)?, logic));
            }
            (Err(_), Ok((new_rest, (comparison, logic)))) => {
                rest = new_rest.trim();
                results.items.push(Entity::Comparison(comparison, logic));
            }
            _ => {
                return Err(anyhow!("Syntax error near '{rest}'"));
            }
        }
    }

    Ok(results)
}
