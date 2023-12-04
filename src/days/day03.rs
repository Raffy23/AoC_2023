use itertools::Itertools;
use std::str;

type Input = Vec<Vec<u8>>;
type Point = (usize, usize);

pub fn solve1(input: Input) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, maybe_symbol)| {
                    !(maybe_symbol.is_ascii_digit() || **maybe_symbol == ('.' as u8))
                })
                .map(|(column_index, _)| {
                    touching_numbers(&input, (row_index, column_index))
                        .iter()
                        .sum::<u32>()
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn solve2(input: Input) -> u32 {
    input
    .iter()
    .enumerate()
    .map(|(row_index, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, maybe_symbol)| {
                **maybe_symbol == ('*' as u8)
            })
            .map(|(column_index, _)| {
                touching_numbers(&input, (row_index, column_index))
            })
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers[0] * numbers[1])
            .sum::<u32>()
    })
    .sum()
}

pub fn parse_input(input: &str) -> Input {
    input.lines().map(|str| str.as_bytes().to_owned()).collect()
}

const LOOKUP_AREA: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn touching_numbers(input: &[Vec<u8>], origin: Point) -> Vec<u32> {
    lookup_digit(input, origin)
        .iter()
        .map(|source| scan_number(input, *source))
        .unique()
        .map(|(_, _, value)| value)
        .collect()
}

fn lookup_digit(input: &[Vec<u8>], origin: Point) -> Vec<Point> {
    LOOKUP_AREA
        .iter()
        .map(|(row, column)| (row + origin.0 as i32, column + origin.1 as i32))
        .filter(|(row, column)| {
            *row >= 0
                && *column >= 0
                && (*row as usize) < input.len()
                && (*column as usize) < input[*row as usize].len()
        })
        .map(|(r, c)| (r as usize, c as usize))
        .filter(|(row, column)| input[*row][*column].is_ascii_digit())
        .collect()
}

fn scan_number(input: &[Vec<u8>], source: Point) -> (usize, usize, u32) {
    let row = &input[source.0 as usize];

    let mut begin: usize = source.1 as usize;
    while begin > 0 && row[begin - 1].is_ascii_digit() {
        begin -= 1;
    }

    let mut end: usize = source.1 as usize;
    while end < row.len() && row[end].is_ascii_digit() {
        end += 1;
    }

    (
        begin,
        end,
        str::from_utf8(&row[begin..end]).unwrap().parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day03::{parse_input, solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT)), 4361)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT)), 467835)
    }
}
