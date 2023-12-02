use std::str::Lines;

pub fn solve1(input: Lines) -> u32 {
    input
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

const NUMBERS: [(&'static str, u32); 9] = [
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

pub fn solve2(input: Lines) -> u32 {
    input
        .map(|line| {
            let mut out: &str = line.into();
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;

            while first.is_none() {
                let first_char = out.chars().next().unwrap();

                if first_char.is_numeric() {
                    first = Some(first_char.to_digit(10).unwrap());
                    break;
                }

                for (needle, value) in NUMBERS.iter() {
                    if out.starts_with(needle) {
                        first = Some(value.clone());
                        break;
                    }
                }

                out = &out[1..];
            }

            while last.is_none() {
                let last_char = out.chars().next_back().unwrap();

                if last_char.is_numeric() {
                    last = Some(last_char.to_digit(10).unwrap());
                    break;
                }

                for (needle, value) in NUMBERS.iter() {
                    if out.ends_with(needle) {
                        last = Some(value.clone());
                        break;
                    }
                }

                out = &out[0..out.len() - 1];
            }

            first.unwrap() * 10 + last.unwrap()
        })
        .sum()
}

#[inline(always)]
pub fn parse_input(input: &str) -> Lines {
    input.lines()
}

#[cfg(test)]
mod tests {
    use crate::day01::{parse_input, solve1, solve2};

    const EXAMPLE_INPUT_1: &'static str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    const EXAMPLE_INPUT_2: &'static str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT_1)), 142)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT_2)), 281)
    }
}
