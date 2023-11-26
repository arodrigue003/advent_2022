use std::num::ParseIntError;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::models::Blueprint;

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map_res(
        tuple((
            tag("Blueprint "),
            digit1,
            tag(": Each ore robot costs "),
            digit1,
            tag(" ore. Each clay robot costs "),
            digit1,
            tag(" ore. Each obsidian robot costs "),
            digit1,
            tag(" ore and "),
            digit1,
            tag(" clay. Each geode robot costs "),
            digit1,
            tag(" ore and "),
            digit1,
            tag(" obsidian."),
            line_ending,
        )),
        |(
            _,
            index,
            _,
            ore_ore,
            _,
            clay_ore,
            _,
            obsidian_ore,
            _,
            obsidian_clay,
            _,
            geode_ore,
            _,
            geode_clay,
            _,
            _,
        )| {
            Ok::<_, ParseIntError>(Blueprint {
                index: usize::from_str(index)?,
                ore: usize::from_str(ore_ore)?,
                clay: usize::from_str(clay_ore)?,
                obsidian: (
                    usize::from_str(obsidian_ore)?,
                    usize::from_str(obsidian_clay)?,
                ),
                geode: (usize::from_str(geode_ore)?, usize::from_str(geode_clay)?),
            })
        },
    )
    .parse(input)
}

pub fn parse_data(data: &str) -> Vec<Blueprint> {
    let (res, blueprints) = many1(parse_blueprint).parse(data).unwrap();
    if res != "" {
        panic!("Unable to fully parse the input: {}", res);
    }

    blueprints
}
