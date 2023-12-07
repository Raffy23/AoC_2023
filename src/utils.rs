use std::{fs::read_to_string, io};

use winnow::{
    ascii::{digit1, multispace0},
    combinator::preceded,
    PResult, Parser,
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

pub fn parse_u32<'s>(input: &mut &'s str) -> PResult<u32> {
    digit1(input).map(|digits| str::parse(digits).unwrap())
}

pub fn parse_u64<'s>(input: &mut &'s str) -> PResult<u64> {
    digit1(input).map(|digits| str::parse(digits).unwrap())
}

pub fn parse_aligned_u32<'s>(input: &mut &'s str) -> PResult<u32> {
    preceded(multispace0, digit1)
        .parse_next(input)
        .map(|digits| str::parse(digits).unwrap())
}
