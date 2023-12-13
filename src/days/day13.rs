use winnow::{
    ascii::newline,
    combinator::{opt, terminated},
    stream::AsChar,
    token::take_till,
    PResult, Parser,
};

type Matrix<'s> = Vec<&'s [u8]>;

pub fn solve1(input: Vec<Matrix>) -> usize {
    solve(input, 0)
}

pub fn solve2(input: Vec<Matrix>) -> usize {
    solve(input, 1)
}

fn solve(input: Vec<Matrix>, delta: u32) -> usize {
    let mut col_values = vec![0_u32; 32];
    let mut row_values = vec![0_u32; 32];

    input
        .into_iter()
        .map(|matrix| {
            col_values.fill(0_u32);
            row_values.fill(0_u32);

            let cols = matrix[0].len();
            let rows = matrix.len();

            for row_index in 0..rows {
                for col_index in 0..cols {
                    let v = if matrix[row_index][col_index] == b'.' { 0 } else { 1 };

                    row_values[row_index] = row_values[row_index] << 1 | v;
                    col_values[col_index] = col_values[col_index] << 1 | v;
                }
            }

            for i in 1..rows {
                if mirrored_delta(&row_values, i, rows) == delta {
                    return i * 100;
                }
            }

            for i in 1..cols {
                if mirrored_delta(&col_values, i, cols) == delta {
                    return i;
                }
            }

            0
        })
        .sum()
}

fn mirrored_delta(hashes: &[u32], offset: usize, hashes_len: usize) -> u32 {
    (0..offset)
        .rev()
        .zip(offset..hashes_len)
        .map(|(left, right)| (hashes[left] ^ hashes[right]).count_ones())
        .sum::<u32>()
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Vec<Matrix<'s>>> {
    let mut values: Vec<Matrix> = Vec::with_capacity(100);

    while let Some(value) = opt(terminated(parse_matrix, newline)).parse_next(input)? {
        values.push(value);
    }

    values.push(parse_matrix(input)?);

    Ok(values)
}

pub fn parse_matrix<'s>(input: &mut &'s str) -> PResult<Matrix<'s>> {
    let mut values: Vec<&'s [u8]> = Vec::with_capacity(32);

    while let Some(value) = opt(terminated(
        take_till(1.., |c: char| c.is_newline()),
        newline,
    ))
    .parse_next(input)?
    {
        values.push(value.as_bytes());
    }

    Ok(values)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day13::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 405)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(13, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 400)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(13, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }

}
