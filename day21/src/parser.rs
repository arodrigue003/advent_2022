use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending, space1};
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::model::{Monkey, Operation, Operator};

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    map(
        alt((tag("+"), tag("-"), tag("*"), tag("/"), tag("%"))),
        From::from,
    )
    .parse(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        tuple((alpha1, space1, parse_operator, space1, alpha1)),
        |(left, _, operator, _, right)| Operation {
            left,
            operator,
            right,
        },
    )
    .parse(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    alt((
        map(parse_operation, |operation| Monkey::Operation(operation)),
        map_res(digit1, |value| {
            Ok::<_, ParseIntError>(Monkey::Value(i64::from_str(value)?))
        }),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Monkey)> {
    map(
        tuple((alpha1, tag(":"), space1, parse_monkey, line_ending)),
        |(monkey_name, _, _, monkey, _)| (monkey_name, monkey),
    )
    .parse(input)
}

pub fn parse_data(data: &str) -> HashMap<&str, Monkey> {
    let (res, monkeys) = many1(parse_line).parse(&data).unwrap();
    if res != "" {
        panic!("Unable to fully parse the input: {}", res);
    }

    monkeys.into_iter().collect()
}
