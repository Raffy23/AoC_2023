use winnow::{
    combinator::{alt, opt, preceded, terminated},
    error::{ErrMode, ErrorKind, ParserError},
    token::tag,
    PResult, Parser, ascii::newline,
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

fn parse_color<'s>(input: &mut &'s str) -> PResult<(u32, u32, u32)> {
    let n = parse_u32(input)?;
    let color = preceded(" ", alt(("blue", "green", "red"))).parse_next(input)?;

    match color {
        "blue" => Ok((0, 0, n)),
        "green" => Ok((0, n, 0)),
        "red" => Ok((n, 0, 0)),
        _ => Err(ErrMode::from_error_kind(input, ErrorKind::Verify)),
    }
}

impl Hand {
    pub fn parse<'s>(input: &mut &'s str) -> PResult<Hand> {
        let mut hand = Hand::default();

        while let Some((red, green, blue)) =
            opt(terminated(preceded(" ", parse_color), opt(','))).parse_next(input)?
        {
            hand.0 += red;
            hand.1 += green;
            hand.2 += blue;
        }

        if hand == Hand::default() {
            Err(ErrMode::from_error_kind(input, ErrorKind::Complete))
        } else {
            Ok(hand)
        }
    }
}

impl Game {
    pub fn parse<'s>(input: &mut &'s str) -> PResult<Game> {
        let _ = tag("Game ").parse_next(input)?;
        let game_id = parse_u32(input)?;
        let _ = tag(":").parse_next(input)?;

        let mut rounds: Vec<Hand> = Vec::with_capacity(10);

        while let Some(round) = opt(terminated(Hand::parse, opt(';'))).parse_next(input)? {
            rounds.push(round)
        }

        Ok(Game(game_id, rounds))
    }
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Vec<Game>> {
    let mut games: Vec<Game> = Vec::with_capacity(100);

    while let Some(round) = opt(terminated(Game::parse, opt(newline))).parse_next(input)? {
        games.push(round)
    }

    Ok(games)
}

#[allow(const_item_mutation)]
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
            parse_input(&mut EXAMPLE_INPUT),
            Ok(
                vec![
                    Game(1, vec![Hand(4, 0, 3), Hand(1, 2, 6), Hand(0, 2, 0)]),
                    Game(2, vec![Hand(0, 2, 1), Hand(1, 3, 4), Hand(0, 1, 1)]),
                    Game(3, vec![Hand(20, 8, 6), Hand(4, 13, 5), Hand(1, 5, 0)]),
                    Game(4, vec![Hand(3, 1, 6), Hand(6, 3, 0), Hand(14, 3, 15)]),
                    Game(5, vec![Hand(6, 3, 1), Hand(1, 2, 2)])
                ]
            )
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 8)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 2286)
    }
}
