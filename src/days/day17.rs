use itertools::Itertools;
use winnow::{
    ascii::newline,
    combinator::{opt, terminated},
    stream::AsChar,
    token::take_till,
    PResult, Parser,
};

type Matrix = Vec<Vec<u8>>;
type Direction = (i8, i8);
type Point = (u8, u8);

use pathfinding::directed::dijkstra::dijkstra;

pub fn solve1(input: Matrix) -> usize {
    shortest_path(input, 0, 3)
}

pub fn solve2(input: Matrix) -> usize {
    shortest_path(input, 4, 10)
}

fn shortest_path(input: Matrix, min_length: u8, max_length: u8) -> usize {
    let rows = input.len() as u8;
    let cols = input[0].len() as u8;

    let start: Point = (0, 0);
    let initial_direction: Direction = (0, 0);
    let end: Point = (rows - 1, cols - 1);

    dijkstra(
        &(start, initial_direction, 0),
        |&(point, dir @ (dir_row, dir_col), path_length)| {
            let mut next = Vec::with_capacity(3);

            if path_length < max_length {
                if let Some(n) = get_neighbor(&input, point, dir, path_length + 1) {
                    next.push(n);
                }
            }

            if path_length == 0 {
                if let Some(n) = get_neighbor(&input, point, (1, 0), 1) {
                    next.push(n);
                }
                if let Some(n) = get_neighbor(&input, point, (0, 1), 1) {
                    next.push(n);
                }
            } else if path_length >= min_length {
                if let Some(n) = get_neighbor(&input, point, (-dir_col, -dir_row), 1) {
                    next.push(n);
                }
                if let Some(n) = get_neighbor(&input, point, (dir_col, dir_row), 1) {
                    next.push(n);
                }
            } 

            next
        },
        |&(pos, _, l)| pos == end && l >= min_length,
    )
    .unwrap()
    .1
}

fn get_neighbor(
    input: &Matrix,
    (row, col): Point,
    direction @ (dir_row, dir_col): Direction,
    length: u8,
) -> Option<((Point, Direction, u8), usize)> {
    let rows = input.len() as i16;
    let cols = input[0].len() as i16;

    let row = row as i16 + dir_row as i16;
    let col = col as i16 + dir_col as i16;

    if row >= 0 && row < rows && col >= 0 && col < cols {
        Some((
            ((row as u8, col as u8), direction, length),
            input[row as usize][col as usize] as usize,
        ))
    } else {
        None
    }
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Matrix> {
    let mut values: Vec<Vec<u8>> = Vec::with_capacity(142);

    while let Some(value) = opt(terminated(
        take_till(1.., |c: char| c.is_newline()),
        newline,
    ))
    .parse_next(input)?
    {
        values.push(
            value
                .as_bytes()
                .into_iter()
                .map(|&c| c - b'0')
                .collect_vec(),
        );
    }

    Ok(values)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day17::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 102)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(17, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 51)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(17, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
