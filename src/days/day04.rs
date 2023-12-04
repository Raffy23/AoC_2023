use std::collections::BTreeSet;

use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};

use crate::utils::parse_aligned_u32;

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
    pub fn parse(input: &str) -> IResult<&str, Card> {
        let (input, _) = tag("Card ")(input)?;
        let (input, id) = parse_aligned_u32(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, left) = separated_list1(tag(" "), parse_aligned_u32)(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, right) = separated_list1(tag(" "), parse_aligned_u32)(input)?;

        Ok((
            input,
            Card(
                id as usize,
                left.into_iter().collect(),
                right.into_iter().collect(),
            ),
        ))
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, games) = separated_list1(newline, Card::parse)(input)?;
    let (input, _) = newline(input)?;

    if input.len() > 0 {
        panic!("Could not fully parse input file");
    }

    Ok((input, games))
}

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
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT).unwrap().1), 13)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT).unwrap().1), 30)
    }
}
