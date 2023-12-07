use std::collections::BTreeSet;

use winnow::{
    ascii::{newline, space0, space1},
    combinator::{opt, preceded, terminated},
    token::tag,
    PResult, Parser,
};

use crate::utils::{parse_aligned_u32, parse_u32};

type ID = usize;

pub struct Card(pub ID, BTreeSet<u32>, BTreeSet<u32>);

pub fn solve1(input: Vec<Card>) -> u32 {
    input
        .into_iter()
        .map(|Card(_, winning_numbers, picked_numbers)| {
            winning_numbers
                .intersection(&picked_numbers)
                .fold(0, |score, _| if score == 0 { 1 } else { score * 2 })
        })
        .sum()
}

pub fn solve2(input: Vec<Card>) -> u32 {
    let mut cards = Vec::<usize>::with_capacity(input.len());
    for i in 0..input.len() {
        cards.insert(i, 1)
    }

    for Card(id, winning_numbers, picked_numbers) in input {
        let won_cards = winning_numbers.intersection(&picked_numbers).count();

        for idx in id..(id + won_cards).min(cards.len()) {
            cards[idx] = cards[idx] + cards[id - 1];
        }
    }

    cards.iter().sum::<usize>() as u32
}

impl Card {
    pub fn parse<'s>(input: &mut &'s str) -> PResult<Card> {
        let _ = tag("Card ").parse_next(input)?;
        let id = parse_aligned_u32(input)?;
        let _ = tag(":").parse_next(input)?;

        let mut left = BTreeSet::new();
        while let Some(number) = opt(preceded(space1, parse_u32)).parse_next(input)? {
            left.insert(number);
        }

        let _ = preceded(space0, '|').parse_next(input)?;

        let mut right = BTreeSet::new();
        while let Some(number) = opt(preceded(space1, parse_u32)).parse_next(input)? {
            right.insert(number);
        }

        Ok(Card(id as usize, left, right))
    }
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Vec<Card>> {
    let mut games: Vec<Card> = Vec::with_capacity(211);
    while let Some(round) = opt(terminated(Card::parse, newline)).parse_next(input)? {
        games.push(round)
    }

    if input.len() > 0 {
        panic!("Could not fully parse input file");
    }

    Ok(games)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day04::{parse_input, solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 13)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 30)
    }
}
