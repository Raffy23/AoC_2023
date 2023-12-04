use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map_res,
    multi::separated_list1, sequence::preceded, IResult,
};

use crate::utils::parse_u32;

pub type Red = u32;
pub type Green = u32;
pub type Blue = u32;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Hand(pub Red, pub Green, pub Blue);

pub type Round = Vec<Hand>;

#[derive(Debug, PartialEq, Clone)]
pub struct Game(pub u32, pub Round);

pub type Games = Vec<Game>;

pub fn solve1(games: Games) -> u32 {
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

pub fn solve2(games: Games) -> u32 {
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

fn parse_color(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, n) = parse_u32(input)?;

    map_res(
        preceded(tag(" "), alt((tag("blue"), tag("green"), tag("red")))),
        move |name| match name {
            "blue" => Ok((0, 0, n)),
            "green" => Ok((0, n, 0)),
            "red" => Ok((n, 0, 0)),
            _ => Err(()),
        },
    )(input)
}

impl Hand {
    pub fn parse(input: &str) -> IResult<&str, Hand> {
        map_res(
            separated_list1(tag(","), preceded(tag(" "), parse_color)),
            |colors| {
                Ok(colors.iter().fold(
                    Hand::default(),
                    |Hand(acc_red, acc_green, acc_blue), (red, green, blue)| {
                        Hand(acc_red + red, acc_green + green, acc_blue + blue)
                    },
                )) as Result<Hand, ()>
            },
        )(input)
    }
}

impl Game {
    pub fn parse(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, game_id) = parse_u32(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, rounds) = separated_list1(tag(";"), Hand::parse)(input)?;

        Ok((input, Game(game_id, rounds)))
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, Game::parse)(input)?;
    let (input, _) = newline(input)?;

    if input.len() > 0 {
        panic!("Could not fully parse input file");
    }

    Ok((input, games))
}

#[cfg(test)]
mod tests {
    use crate::day02::{parse_input, solve1, solve2, Game, Hand};

    const EXAMPLE_INPUT: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn input_parsing() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            Ok((
                "",
                vec![
                    Game(1, vec![Hand(4, 0, 3), Hand(1, 2, 6), Hand(0, 2, 0)]),
                    Game(2, vec![Hand(0, 2, 1), Hand(1, 3, 4), Hand(0, 1, 1)]),
                    Game(3, vec![Hand(20, 8, 6), Hand(4, 13, 5), Hand(1, 5, 0)]),
                    Game(4, vec![Hand(3, 1, 6), Hand(6, 3, 0), Hand(14, 3, 15)]),
                    Game(5, vec![Hand(6, 3, 1), Hand(1, 2, 2)])
                ]
            ))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT).unwrap().1), 8)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT).unwrap().1), 2286)
    }
}
