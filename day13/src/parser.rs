use std::num::ParseIntError;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, terminated, tuple};
use nom::{IResult, Parser};

use crate::model::{PacketInner, PacketPair};

pub fn parse_packet_inner(input: &str) -> IResult<&str, PacketInner> {
    alt((
        map_res(digit1, |data| {
            Ok::<_, ParseIntError>(PacketInner::Value(i32::from_str(data)?))
        }),
        map(
            delimited(
                tag("["),
                separated_list0(tag(","), parse_packet_inner),
                tag("]"),
            ),
            PacketInner::List,
        ),
    ))
    .parse(input)
}

pub fn parse_packet_pair(input: &str) -> IResult<&str, PacketPair> {
    map(
        tuple((
            terminated(parse_packet_inner, line_ending),
            terminated(parse_packet_inner, line_ending),
            opt(line_ending),
        )),
        |(left, right, _)| PacketPair { left, right },
    )
    .parse(input)
}

pub fn parse_data(data: &str) -> Vec<PacketPair> {
    let (res, packet_pairs) = many0(parse_packet_pair).parse(data).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    packet_pairs
}
