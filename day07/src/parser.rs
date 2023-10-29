use std::num::ParseIntError;
use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::alphanumeric1;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, line_ending, multispace1},
    combinator::{map, map_res},
    sequence::{delimited, pair, terminated, tuple},
    IResult, Parser,
};

use crate::model::{CdTarget, Command, DirEntry, FileEntry, Inode};

#[inline]
fn is_alphanumeric_dot(c: char) -> bool {
    c.is_alphanumeric() || c == '.'
}

pub fn parse_dir_line(input: &str) -> IResult<&str, DirEntry> {
    map(
        delimited(
            pair(tag("dir"), multispace1),
            take_while1(is_alphanumeric_dot),
            line_ending,
        ),
        |name| DirEntry { name, size: 0 },
    )
    .parse(input)
}

pub fn parse_file_line(input: &str) -> IResult<&str, FileEntry> {
    map_res(
        terminated(
            tuple((digit1, multispace1, take_while1(is_alphanumeric_dot))),
            line_ending,
        ),
        |(size, _, name)| {
            Ok::<_, ParseIntError>(FileEntry {
                name,
                size: i64::from_str(size)?,
            })
        },
    )
    .parse(input)
}

pub fn parse_ls_line(input: &str) -> IResult<&str, Inode> {
    alt((
        map(parse_dir_line, |res| Inode::Dir(res)),
        map(parse_file_line, |res| Inode::File(res)),
    ))
    .parse(input)
}

pub fn parse_ls_command(input: &str) -> IResult<&str, Vec<Inode>> {
    preceded(
        tuple((tag("$"), multispace1, tag("ls"), line_ending)),
        many1(parse_ls_line),
    )
    .parse(input)
}

pub fn parse_cd_command(input: &str) -> IResult<&str, CdTarget> {
    delimited(
        tuple((tag("$"), multispace1, tag("cd"), multispace1)),
        alt((
            map(tag("/"), |_| CdTarget::Root),
            map(tag(".."), |_| CdTarget::Up),
            map(alphanumeric1, |res| CdTarget::Directory(res)),
        )),
        line_ending,
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    many1(alt((
        map(parse_ls_command, |res| Command::Ls(res)),
        map(parse_cd_command, |res| Command::Cd(res)),
    )))
    .parse(input)
}

pub fn parse_data(data: &str) -> Vec<Command> {
    let (res, commands) = parse_input(&data).unwrap();
    if res != "" {
        panic!("Unable to fully parse the input: {}", res);
    }

    commands
}
