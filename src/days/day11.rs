use itertools::Itertools;

type Point = (u64, u64);
type Input = (Vec<(Point, u8)>, Vec<u8>);

pub fn solve1(input: Input) -> u64 {
    expand_universe(input, 1)
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(a, b))
        .sum()
}

pub fn solve2(input: Input) -> u64 {
    expand_universe(input, 1_000_000 - 1)
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(a, b))
        .sum()
}

fn expand_universe((galaxies, fixed_cols_before): Input, steps: u64) -> Vec<Point> {
    galaxies
        .into_iter()
        .map(|((row_index, col_index), fixed_rows_before)| {
            (
                row_index + steps * (row_index - fixed_rows_before as u64),
                col_index + steps * (col_index - fixed_cols_before[col_index as usize] as u64),
            )
        })
        .collect_vec()
}

pub fn parse_input(input: &str) -> Input {
    let mut result = Vec::with_capacity(65);
    let lines = input.lines().collect_vec();

    let mut fixed_rows = 0_u8;
    let mut fixed_cols_flag = vec![false; lines.len()];

    for (row_index, row) in lines.into_iter().enumerate() {
        let mut has_galaxy = false;

        for (col_index, &symbol) in row.as_bytes().into_iter().enumerate() {
            if symbol == b'#' {
                has_galaxy = true;
                fixed_cols_flag[col_index] = true;
                result.push(((row_index as u64, col_index as u64), fixed_rows));
            }
        }

        if has_galaxy {
            fixed_rows += 1;
        }
    }

    let mut fixed_cols_before = vec![0_u8; fixed_cols_flag.len()];
    for i in 1..fixed_cols_before.len() {
        fixed_cols_before[i] =
            fixed_cols_before[i - 1] + if fixed_cols_flag[i - 1] { 1 } else { 0 };
    }

    (result, fixed_cols_before)
}

fn manhattan_distance((x1, y1): Point, (x2, y2): Point) -> u64 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day11::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT)), 374)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(11, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(input.as_str())))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT)), 82000210)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(11, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(input.as_str())))
    }
}
