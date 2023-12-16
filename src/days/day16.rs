use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::collections::VecDeque;
use winnow::{
    ascii::newline,
    combinator::{opt, terminated},
    stream::AsChar,
    token::take_till,
    PResult, Parser,
};

type Matrix = Vec<Vec<u8>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn next(&self, row: usize, col: usize) -> (i64, i64) {
        match self {
            Direction::Up => (row as i64 - 1, col as i64),
            Direction::Left => (row as i64, col as i64 - 1),
            Direction::Down => (row as i64 + 1, col as i64),
            Direction::Right => (row as i64, col as i64 + 1),
        }
    }

    pub fn bit_mask(&self) -> u8 {
        match self {
            Direction::Up => 0b00000001,
            Direction::Left => 0b00000010,
            Direction::Down => 0b00000100,
            Direction::Right => 0b00001000,
        }
    }
}

pub fn solve1(input: Matrix) -> usize {
    simulate_beam(&input, (0, 0, Direction::Right))
}

pub fn solve2(input: Matrix) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    (0..rows)
        .map(|row| (row, 0, Direction::Right))
        .chain((0..rows).map(|row| (row, cols - 1, Direction::Left)))
        .chain((0..cols).map(|col| (0, col, Direction::Down)))
        .chain((0..cols).map(|col| (rows - 1, col, Direction::Up)))
        .par_bridge()
        .map_with(new_visited(&input), |cache, beam| {
            clear_visited(cache);
            _simulate_beam(&input, beam, cache)
        })
        .max()
        .unwrap()
}

#[inline]
fn simulate_beam(input: &Matrix, beam: (usize, usize, Direction)) -> usize {
    _simulate_beam(input, beam, &mut new_visited(input))
}

#[inline]
fn new_visited(input: &Matrix) -> Vec<Vec<u8>> {
    vec![vec![0_u8; input[0].len()]; input.len()]
}

#[inline]
fn clear_visited(visited: &mut Vec<Vec<u8>>) {
    for row in visited {
        row.fill(0_u8);
    }
}

fn _simulate_beam(
    input: &Matrix,
    beam: (usize, usize, Direction),
    visited: &mut Vec<Vec<u8>>,
) -> usize {
    let mut beams = VecDeque::with_capacity(32);

    let rows = input.len();
    let cols = input[0].len();

    let is_in_bounds = move |(row, col): (i64, i64)| {
        row >= 0 && col >= 0 && row < rows as i64 && col < cols as i64
    };

    let enqueue_beam = move |beams: &mut VecDeque<(usize, usize, Direction)>,
                             point @ (row, col): (i64, i64),
                             direction: Direction| {
        if is_in_bounds(point) {
            beams.push_back((row as usize, col as usize, direction));
        }
    };

    beams.push_back(beam);

    while let Some((row, col, direction)) = beams.pop_front() {
        if visited[row][col] & direction.bit_mask() > 0 {
            continue;
        }

        visited[row][col] |= direction.bit_mask();

        match input[row][col] {
            b'.' => enqueue_beam(&mut beams, direction.next(row, col), direction),
            b'\\' => match direction {
                Direction::Up => {
                    enqueue_beam(&mut beams, (row as i64, col as i64 - 1), Direction::Left)
                }
                Direction::Left => {
                    enqueue_beam(&mut beams, (row as i64 - 1, col as i64), Direction::Up)
                }
                Direction::Down => {
                    enqueue_beam(&mut beams, (row as i64, col as i64 + 1), Direction::Right)
                }
                Direction::Right => {
                    enqueue_beam(&mut beams, (row as i64 + 1, col as i64), Direction::Down)
                }
            },
            b'/' => match direction {
                Direction::Up => {
                    enqueue_beam(&mut beams, (row as i64, col as i64 + 1), Direction::Right)
                }
                Direction::Left => {
                    enqueue_beam(&mut beams, (row as i64 + 1, col as i64), Direction::Down)
                }
                Direction::Down => {
                    enqueue_beam(&mut beams, (row as i64, col as i64 - 1), Direction::Left)
                }
                Direction::Right => {
                    enqueue_beam(&mut beams, (row as i64 - 1, col as i64), Direction::Up)
                }
            },
            b'|' if direction == Direction::Up || direction == Direction::Down => {
                enqueue_beam(&mut beams, direction.next(row, col), direction)
            }
            b'|' => {
                enqueue_beam(&mut beams, (row as i64 - 1, col as i64), Direction::Up);
                enqueue_beam(&mut beams, (row as i64 + 1, col as i64), Direction::Down);
            }
            b'-' if direction == Direction::Right || direction == Direction::Left => {
                enqueue_beam(&mut beams, direction.next(row, col), direction)
            }
            b'-' => {
                enqueue_beam(&mut beams, (row as i64, col as i64 - 1), Direction::Left);
                enqueue_beam(&mut beams, (row as i64, col as i64 + 1), Direction::Right);
            }
            _ => unreachable!(),
        }
    }

    visited
        .into_iter()
        .map(|line| line.into_iter().filter(|&&mut p| p > 0).count())
        .sum()
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Matrix> {
    let mut values: Vec<Vec<u8>> = Vec::with_capacity(100);

    while let Some(value) = opt(terminated(
        take_till(1.., |c: char| c.is_newline()),
        newline,
    ))
    .parse_next(input)?
    {
        values.push(value.into());
    }

    Ok(values)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day16::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 46)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(16, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 51)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(16, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
