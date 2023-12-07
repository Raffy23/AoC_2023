use itertools::Itertools;
use winnow::{
    ascii::{multispace1, space1},
    combinator::{opt, terminated},
    token::tag,
    PResult, Parser,
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

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Input> {
    let _ = tag("Time: ").parse_next(input)?;
    let _ = space1(input)?;

    let mut times: Vec<u64> = Vec::with_capacity(4);
    while let Some(value) = opt(terminated(parse_u64, multispace1)).parse_next(input)? {
        times.push(value);
    }

    let _ = tag("Distance: ").parse_next(input)?;
    let _ = space1(input)?;

    let mut distances: Vec<u64> = Vec::with_capacity(4);
    while let Some(value) = opt(terminated(parse_u64, multispace1)).parse_next(input)? {
        distances.push(value);
    }

    Ok(Input(times, distances))
}

#[allow(const_item_mutation)]
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
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 288)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(6, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 71503)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(6, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
