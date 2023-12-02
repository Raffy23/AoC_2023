use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::newline, streaming::digit1},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::fs::read_to_string;

fn main() {
    println!("Day 02 Part 1: {}", solve1());
    println!("Day 02 Part 2: {}", solve2());
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}
impl Color {
    pub fn parse(input: &str) -> IResult<&str, Color> {
        let (input, n) = parse_u32(input)?;

        map_res(
            preceded(tag(" "), alt((tag("blue"), tag("green"), tag("red")))),
            move |name| match name {
                "blue" => Ok(Color::Blue(n)),
                "green" => Ok(Color::Green(n)),
                "red" => Ok(Color::Red(n)),
                _ => Err(()),
            },
        )(input)
    }
}

struct Hand(u32, u32, u32);
impl Hand {
    pub fn parse(input: &str) -> IResult<&str, Hand> {
        map_res(
            separated_list1(tag(","), preceded(tag(" "), Color::parse)),
            |colors| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                for color in colors {
                    match color {
                        Color::Red(n) => red = n,
                        Color::Green(n) => green = n,
                        Color::Blue(n) => blue = n,
                    }
                }

                Ok(Hand(red, green, blue)) as Result<Hand, ()>
            },
        )(input)
    }
}

type Round = Vec<Hand>;
struct Game(u32, Round);
impl Game {
    pub fn parse(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, game_id) = parse_u32(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, rounds) = separated_list1(tag(";"), Hand::parse)(input)?;

        Ok((input, Game(game_id, rounds)))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, Game::parse)(input)?;
    let (input, _) = newline(input)?;

    if input.len() > 0 {
        panic!("Could not fully parse input file");
    }

    Ok((input, games))
}

pub fn solve1() -> u32 {
    let input = read_to_string("./input/day02_01.txt").expect("Unable to read input file");
    let (_, games) = parse_input(&input).unwrap();

    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    games
        .iter()
        .filter(|game| {
            !game.1.iter().any(|Hand(red, green, blue)| {
                *red > max_red || *green > max_green || *blue > max_blue
            })
        })
        .map(|game| game.0)
        .sum()
}

pub fn solve2() -> u32 {
    let input = read_to_string("./input/day02_01.txt").expect("Unable to read input file");
    let (_, games) = parse_input(&input).unwrap();

    games
        .iter()
        .map(|game| {
            game.1.iter().fold(
                (0, 0, 0),
                |(max_red, max_green, max_blue), Hand(red, green, blue)| {
                    (
                        max_red.max(*red),
                        max_green.max(*green),
                        max_blue.max(*blue),
                    )
                },
            )
        })
        .map(|colors| colors.0 * colors.1 * colors.2)
        .sum()
}
