use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::utils::parse_u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

pub fn solve1(input: Vec<(&str, u32)>) -> u64 {
    input
        .into_iter()
        .map(|(hand, score)| {
            let hand_bytes = hand.as_bytes();
            let hand = [
                Card::new(hand_bytes[0]),
                Card::new(hand_bytes[1]),
                Card::new(hand_bytes[2]),
                Card::new(hand_bytes[3]),
                Card::new(hand_bytes[4]),
            ];

            let current_type = hand
                .iter()
                .counts()
                .into_iter()
                .fold(HandType::HighCard, |current_type, (_, count)| {
                    current_type.next_with_count(count)
                });

            (hand, current_type, score)
        })
        .sorted_by(sort_cards)
        .enumerate()
        .map(|(index, (_, _, score))| (index as u64 + 1) * score as u64)
        .sum()
}

pub fn solve2(input: Vec<(&str, u32)>) -> u64 {
    input
        .into_iter()
        .map(|(hand, score)| {
            let hand_bytes = hand.as_bytes();
            let hand = [
                Card::new_with_joker(hand_bytes[0]),
                Card::new_with_joker(hand_bytes[1]),
                Card::new_with_joker(hand_bytes[2]),
                Card::new_with_joker(hand_bytes[3]),
                Card::new_with_joker(hand_bytes[4]),
            ];

            let counts = hand.iter().counts();
            let jokers = *counts.get(&Card::Joker).unwrap_or(&0);

            let mut current_type =
                counts
                    .into_iter()
                    .fold(HandType::HighCard, |current_type, (card, count)| {
                        if *card == Card::Joker {
                            current_type
                        } else {
                            current_type.next_with_count(count)
                        }
                    });

            for _ in 0..jokers {
                current_type = current_type.next_with_joker();
            }

            (hand, current_type, score)
        })
        .sorted_by(sort_cards)
        .enumerate()
        .map(|(index, (_, _, score))| (index as u64 + 1) * score as u64)
        .sum()
}

fn sort_cards<'a, 'b>(
    (hand_left, type_left, _): &'a ([Card; 5], HandType, u32),
    (hand_right, type_right, _): &'b ([Card; 5], HandType, u32),
) -> Ordering {
    let type_cmp = type_left.cmp(type_right);

    if type_cmp == Ordering::Equal {
        hand_left.cmp(hand_right)
    } else {
        type_cmp
    }
}

impl Card {
    pub fn new(value: u8) -> Card {
        match value {
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::J,
            b'T' => Card::T,
            v => Card::Number(v - b'0'),
        }
    }

    pub fn new_with_joker(value: u8) -> Card {
        match value {
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::Joker,
            b'T' => Card::T,
            v => Card::Number(v - b'0'),
        }
    }
}

impl HandType {
    pub fn next_with_joker(self) -> HandType {
        match self {
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::ThreeOfAKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
            HandType::FullHouse => HandType::FullHouse,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FiveOfAKind => HandType::FiveOfAKind,
        }
    }

    pub fn next_with_count(self, count: usize) -> HandType {
        match count {
            1 => self,
            2 if self == HandType::HighCard => HandType::OnePair,
            2 if self == HandType::OnePair => HandType::TwoPair,
            2 if self == HandType::ThreeOfAKind => HandType::FullHouse,
            3 if self == HandType::HighCard => HandType::ThreeOfAKind,
            3 if self == HandType::OnePair => HandType::FullHouse,
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => panic!("unknown hand composition"),
        }
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
    terminated(
        separated_list1(newline, separated_pair(alphanumeric1, tag(" "), parse_u32)),
        newline,
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        day07::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT).unwrap().1), 6440)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(7, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&input).unwrap().1))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT).unwrap().1), 5905)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(7, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&input).unwrap().1))
    }
}
