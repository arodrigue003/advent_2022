use crate::game::Command;
use crate::game::Rotation;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::{IResult, Parser};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn parse_command_line(input: &str) -> IResult<&str, Vec<Command>> {
    many1(alt((
        map_res(digit1, |value: &str| {
            Ok::<_, ParseIntError>(Command::Forward(usize::from_str(value)?))
        }),
        map(tag("R"), |_| Command::Rotate(Rotation::Right)),
        map(tag("L"), |_| Command::Rotate(Rotation::Left)),
    )))
    .parse(input)
}
