use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

use crate::utils::parse_u64;

pub struct Input(pub Vec<u64>, pub Vec<u64>);

pub fn solve1(Input(times, distances): Input) -> u64 {
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| {
            let (x1, x2) = solve_limits(time, distance);
            (x2 + 1) - x1
        })
        .product()
}

pub fn solve2(Input(times, distances): Input) -> u64 {
    let numbers_to_number = move |numbers: Vec<u64>| {
        numbers
        .into_iter()
        .map(|num| num.to_string())
        .join("")
        .parse::<u64>()
        .unwrap()
    };

    let time = numbers_to_number(times);
    let distance = numbers_to_number(distances);

    let (x1, x2) = solve_limits(time, distance);
    (x2 + 1) - x1
}

fn solve_limits(time: u64, distance: u64) -> (u64, u64) {
    const A: f64 = -1.0;
    let b = time as f64;
    let c = -(distance as f64);

    let f = move |x: f64| -> f64 { A * x * x + b * x + c };

    let d = (b * b - 4.0 * A * c).sqrt();
    let x1 = (-b + d) / (2.0 * A);
    let x2 = (-b - d) / (2.0 * A);

    let x1 = if x1.fract() == 0.0 && f(x1) == 0.0 {
        (x1 + 1.0).ceil()
    } else {
        x1.ceil()
    } as u64;

    let x2 = if x2.fract() == 0.0 && f(x2) == 0.0 {
        (x2 - 1.0).floor()
    } else {
        x2.floor()
    } as u64;

    (x1, x2)
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, times) = terminated(separated_list1(multispace1, parse_u64), newline)(input)?;
    let (input, _) = tag("Distance: ")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distances) = terminated(separated_list1(multispace1, parse_u64), newline)(input)?;

    Ok((input, Input(times, distances)))
}

#[cfg(test)]
mod tests {
    use crate::{
        day06::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT).unwrap().1), 288)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(6, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&input).unwrap().1))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT).unwrap().1), 71503)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(6, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&input).unwrap().1))
    }
}
