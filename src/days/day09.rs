use winnow::{
    ascii::{newline, space0},
    combinator::{opt, terminated},
    PResult, Parser,
};

use crate::utils::parse_i32;

pub fn solve1(input: Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .map(|data| lagrange_interpolate(&data, data.len() as i32))
        .sum()
}

pub fn solve2(input: Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .map(|mut data| {
            data.reverse();
            lagrange_interpolate(&data, data.len() as i32)
        })
        .sum()
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Vec<Vec<i32>>> {
    let mut lines: Vec<Vec<i32>> = Vec::with_capacity(200);

    while let Some(line) = opt(terminated(parse_line, newline)).parse_next(input)? {
        lines.push(line);
    }

    Ok(lines)
}

pub fn parse_line<'s>(input: &mut &'s str) -> PResult<Vec<i32>> {
    let mut values: Vec<i32> = Vec::with_capacity(21);

    while let Some(value) = opt(terminated(parse_i32, space0)).parse_next(input)? {
        values.push(value);
    }

    Ok(values)
}

fn lagrange_interpolate(y: &[i32], xi: i32) -> i32 {
    let mut result: f64 = 0.0;

    for x_i in 0..y.len() {
        let mut term: f64 = y[x_i] as f64;

        for x_j in 0..y.len() {
            if x_i != x_j {
                term *= (xi - x_j as i32) as f64 / (x_i as i32 - x_j as i32) as f64;
            }
        }

        result += term;
    }

    result.round() as i32
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day09::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 114)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(9, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 2)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(9, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
