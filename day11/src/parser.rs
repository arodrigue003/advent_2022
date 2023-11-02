use crate::model::{Monkey, Operand, Operation, Operator, Test};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::{IResult, Parser};
use std::str::FromStr;
use std::usize;

pub fn parse_starting_items_line(input: &str) -> IResult<&str, Vec<i64>> {
    map(
        delimited(
            tuple((space0, tag("Starting items:"), space0)),
            pair(digit1, many0(preceded(pair(tag(","), space0), digit1))),
            line_ending,
        ),
        |(first, remaining): (&str, Vec<&str>)| {
            std::iter::once(first)
                .chain(remaining.into_iter())
                .map(|elt| elt.parse().unwrap())
                .collect()
        },
    )
    .parse(input)
}

pub fn parse_operation_line(input: &str) -> IResult<&str, Operation> {
    map(
        delimited(
            tuple((space0, tag("Operation: new ="), space0)),
            tuple((
                alt((tag("old"), digit1)),
                space1,
                alt((tag("+"), tag("-"), tag("*"), tag("/"), tag("%"))),
                space1,
                alt((tag("old"), digit1)),
            )),
            line_ending,
        ),
        |(left, _, operator, _, right)| Operation {
            left: Operand::from(left),
            operator: Operator::from(operator),
            right: Operand::from(right),
        },
    )
    .parse(input)
}

fn parse_test_line(input: &str) -> IResult<&str, i64> {
    map_res(
        delimited(
            tuple((space0, tag("Test: divisible by"), space0)),
            digit1,
            line_ending,
        ),
        |val| i64::from_str(val),
    )
    .parse(input)
}

fn parse_test_true_line(input: &str) -> IResult<&str, usize> {
    map_res(
        delimited(
            tuple((space0, tag("If true: throw to monkey"), space0)),
            digit1,
            line_ending,
        ),
        |val| usize::from_str(val),
    )
    .parse(input)
}

fn parse_test_false_line(input: &str) -> IResult<&str, usize> {
    map_res(
        delimited(
            tuple((space0, tag("If false: throw to monkey"), space0)),
            digit1,
            line_ending,
        ),
        |val| usize::from_str(val),
    )
    .parse(input)
}

pub fn parse_test_lines(input: &str) -> IResult<&str, Test> {
    map(
        tuple((parse_test_line, parse_test_true_line, parse_test_false_line)),
        |(quotient, true_target, false_target)| Test {
            quotient,
            true_target,
            false_target,
        },
    )
    .parse(input)
}

pub fn parse_monkey_line(input: &str) -> IResult<&str, usize> {
    map_res(
        delimited(
            tuple((space0, tag("Monkey"), space0)),
            digit1,
            pair(tag(":"), line_ending),
        ),
        |val| usize::from_str(val),
    )
    .parse(input)
}

pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((parse_monkey_line, parse_starting_items_line, parse_operation_line, parse_test_lines, many0(line_ending))),
        |(number, items, operation, test, _)| {
            Monkey {
                number,
                items: items.into(),
                operation,
                test,
                inspected_items: 0,
            }
        }
    ).parse(input)
}

pub fn parse_data(data: &str) -> Vec<Monkey> {
    let (res, monkeys) = many1(parse_monkey).parse(data).unwrap();
    if res != "" {
        panic!("Unable to fully parse the input: {}", res);
    }

    monkeys
}
