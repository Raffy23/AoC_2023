use hashbrown::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use winnow::{
    ascii::newline,
    combinator::{opt, terminated},
    stream::AsChar,
    token::take_till,
    PResult, Parser,
};

type Matrix = Vec<Vec<u8>>;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

const ROUND_ROCK: u8 = b'O';
const CUBE_ROCK: u8 = b'#';
const EMPTY_SPACE: u8 = b'.';

pub fn solve1(mut input: Matrix) -> usize {
    let rows = input.len();

    move_rocks(&mut input, Direction::North);

    input
        .into_iter()
        .enumerate()
        .map(|(row_index, line)| {
            let rocks = line.into_iter().filter(|&c| c == ROUND_ROCK).count();
            rocks * (rows - row_index)
        })
        .sum()
}

pub fn solve2(mut input: Matrix) -> usize {
    const TARGET_CYCLE: usize = 1_000_000_000;

    let rows = input.len();

    let mut seen = HashMap::with_capacity(164);
    let mut cycle = 1;
    let mut cycle_length = 0;

    while cycle < TARGET_CYCLE {
        move_rocks(&mut input, Direction::North);
        move_rocks(&mut input, Direction::West);
        move_rocks(&mut input, Direction::South);
        move_rocks(&mut input, Direction::East);

        let hash = calculate_hash(&mut input);
        if let Some(&length) = seen.get(&hash) {
            cycle_length = cycle - length;
            break;
        }

        seen.insert(hash, cycle);
        cycle += 1;
    }

    let remaining = (TARGET_CYCLE - cycle) % cycle_length;
    for _ in 0..remaining {
        move_rocks(&mut input, Direction::North);
        move_rocks(&mut input, Direction::West);
        move_rocks(&mut input, Direction::South);
        move_rocks(&mut input, Direction::East);
    }

    input
        .into_iter()
        .enumerate()
        .map(|(row_index, line)| {
            let rocks = line.into_iter().filter(|&c| c == ROUND_ROCK).count();
            rocks * (rows - row_index)
        })
        .sum()
}

fn move_rocks(input: &mut Matrix, direction: Direction) {
    let rows = input.len();
    let cols = input[0].len();

    match direction {
        Direction::North => {
            for col_index in 0..cols {
                let mut top_index = 0;

                for row_index in 0..rows {
                    match input[row_index][col_index] {
                        ROUND_ROCK if top_index == row_index => {
                            top_index += 1;
                        }
                        ROUND_ROCK => {
                            input[top_index][col_index] = ROUND_ROCK;
                            input[row_index][col_index] = EMPTY_SPACE;
                            top_index += 1;
                        }
                        CUBE_ROCK => {
                            top_index = row_index + 1;
                        }
                        _ => { /* nothing */ }
                    }
                }
            }
        }

        Direction::South => {
            for col_index in 0..cols {
                let mut top_index = rows - 1;

                for row_index in (0..rows).rev() {
                    match input[row_index][col_index] {
                        ROUND_ROCK if top_index == row_index => {
                            top_index = top_index.checked_sub(1).unwrap_or(0);
                        }
                        ROUND_ROCK => {
                            input[top_index][col_index] = ROUND_ROCK;
                            input[row_index][col_index] = EMPTY_SPACE;
                            top_index -= 1;
                        }
                        CUBE_ROCK if row_index > 0 => {
                            top_index = row_index - 1;
                        }
                        _ => { /* nothing */ }
                    }
                }
            }
        }

        Direction::West => {
            for row_index in 0..rows {
                let mut top_index = 0;

                for col_index in 0..cols {
                    match input[row_index][col_index] {
                        ROUND_ROCK if top_index == col_index => {
                            top_index += 1;
                        }
                        ROUND_ROCK => {
                            input[row_index][top_index] = ROUND_ROCK;
                            input[row_index][col_index] = EMPTY_SPACE;
                            top_index += 1;
                        }
                        CUBE_ROCK => {
                            top_index = col_index + 1;
                        }
                        _ => { /* nothing */ }
                    }
                }
            }
        }

        Direction::East => {
            for row_index in 0..rows {
                let mut top_index = rows - 1;

                for col_index in (0..cols).rev() {
                    match input[row_index][col_index] {
                        ROUND_ROCK if top_index == col_index => {
                            top_index = top_index.checked_sub(1).unwrap_or(0);
                        }
                        ROUND_ROCK => {
                            input[row_index][top_index] = ROUND_ROCK;
                            input[row_index][col_index] = EMPTY_SPACE;
                            top_index -= 1;
                        }
                        CUBE_ROCK if col_index > 0 => {
                            top_index = col_index - 1;
                        }
                        _ => { /* nothing */ }
                    }
                }
            }
        }
    }
}

#[inline]
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
        day14::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 136)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(14, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 64)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(14, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
