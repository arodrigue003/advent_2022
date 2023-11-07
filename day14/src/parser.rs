use crate::model::{Line, Point};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::{IResult, Parser};
use std::num::ParseIntError;
use std::str::FromStr;

fn parse_point(input: &str) -> IResult<&str, Point> {
    map_res(tuple((digit1, tag(","), digit1)), |(column, _, line)| {
        Ok::<_, ParseIntError>(Point {
            line: usize::from_str(line)?,
            column: usize::from_str(column)?,
        })
    })
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        terminated(
            pair(
                parse_point,
                many0(preceded(tuple((space1, tag("->"), space1)), parse_point)),
            ),
            line_ending,
        ),
        |(first, remaining)| Line(std::iter::once(first).chain(remaining).collect()),
    )
    .parse(input)
}

pub fn parse_data(data: &str) -> Vec<Line> {
    let (res, lines) = many1(parse_line).parse(data).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    lines
}
