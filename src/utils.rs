use std::{fs::read_to_string, io};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    sequence::preceded,
    IResult,
};

const INPUT_FOLDER: &'static str = "./input";

pub enum Part {
    Example,
    Part1,
    Part2,
}

impl Part {
    fn extension(self) -> &'static str {
        match self {
            Part::Example => "example",
            Part::Part1 => "01",
            Part::Part2 => "02",
        }
    }
}

pub fn read_input(day: u8, part: Part) -> io::Result<String> {
    read_to_string(format!(
        "{}/day{:0>2}_{}.txt",
        INPUT_FOLDER,
        day,
        part.extension()
    ))
}

pub fn parse_u32<'a>(input: &'a str) -> IResult<&'a str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_u64<'a>(input: &'a str) -> IResult<&'a str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_aligned_u32(input: &str) -> IResult<&str, u32> {
    map_res(
        alt((
            digit1,
            preceded(tag(" "), digit1),
            preceded(tag("  "), digit1),
        )),
        str::parse,
    )(input)
}
