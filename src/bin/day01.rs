use std::fs::read_to_string;

fn main() {
    println!("Day 01 Part 1: {}", solve1());
    println!("Day 01 Part 2: {}", solve2());
}

pub fn solve1() -> u32 {
    read_to_string("./input/day01_01.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_numeric())
                .expect("can't find first number")
                .to_digit(10)
                .expect("first number not in base 10");

            let last = line
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .expect("can't find last number")
                .to_digit(10)
                .expect("last number not in base 10");

            first * 10 + last
        })
        .sum()
}

pub fn solve2() -> u32 {
    read_to_string("./input/day01_01.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut out: String = line.into();
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;

            let r = vec![
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];

            while first.is_none() {
                let first_char = out.chars().next().unwrap();

                if first_char.is_numeric() {
                    first = Some(first_char.to_digit(10).unwrap());
                    break;
                }

                for (needle, value) in r.iter() {
                    if out.starts_with(needle) {
                        first = Some(*value);
                        break;
                    }
                }

                out = out.get(1..).unwrap().into();
            }

            while last.is_none() {
                let last_char = out.chars().next_back().unwrap();

                if last_char.is_numeric() {
                    last = Some(last_char.to_digit(10).unwrap());
                    break;
                }

                for (needle, value) in r.iter() {
                    if out.ends_with(needle) {
                        last = Some(*value);
                        break;
                    }
                }

                out = out.get(0..out.len() - 1).unwrap().into();
            }

            first.unwrap() * 10 + last.unwrap()
        })
        .sum()
}
