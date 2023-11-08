use crate::sensor_scan::{Point, SensorScan};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;
use std::str::FromStr;

#[inline]
fn is_digit_minus(c: char) -> bool {
    c.is_ascii_digit() || c == '-'
}

fn parse_line(input: &str) -> IResult<&str, SensorScan> {
    map_res(
        tuple((
            tag("Sensor at x="),
            take_while1(is_digit_minus),
            tag(", y="),
            take_while1(is_digit_minus),
            tag(": closest beacon is at x="),
            take_while1(is_digit_minus),
            tag(", y="),
            take_while1(is_digit_minus),
            line_ending,
        )),
        |(_, sensor_x, _, sensor_y, _, beacon_x, _, beacon_y, _)| {
            Ok::<_, ParseIntError>(SensorScan::new(
                Point {
                    x: i64::from_str(sensor_x)?,
                    y: i64::from_str(sensor_y)?,
                },
                Point {
                    x: i64::from_str(beacon_x)?,
                    y: i64::from_str(beacon_y)?,
                },
            ))
        },
    )
    .parse(input)
}

pub fn parse_data(data: &str) -> Vec<SensorScan> {
    let (res, sensor_scans) = many1(parse_line).parse(data).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    sensor_scans
}
