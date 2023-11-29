use std::num::ParseIntError;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::models::Valve;

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    map_res(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            digit1,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
            line_ending,
        )),
        |(_, name, _, flow_rate, _, direction, _): (_, &str, _, _, _, _, _)| {
            Ok::<_, ParseIntError>(Valve {
                name: name.to_string(),
                flow_rate: usize::from_str(flow_rate)?,
                direction: direction.into_iter().map(From::from).collect(),
            })
        },
    )
    .parse(input)
}

pub fn parse_data(data: &str) -> Vec<Valve> {
    let (res, valves) = many1(parse_valve).parse(data).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    valves
}
