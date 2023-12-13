use hashbrown::HashMap;
use itertools::Itertools;
use winnow::{
    ascii::newline,
    combinator::{opt, terminated},
    token::{tag, take_until0},
    PResult, Parser,
};

use crate::utils::parse_u32;

type Input<'s> = Vec<(&'s [u8], Vec<u8>)>;
type Cache = HashMap<(u8, u8, u8), u64>;

pub fn solve1(input: Input) -> u64 {
    let mut cache = Cache::new();

    input
        .into_iter()
        .map(|(spring_conditions, windows)| {
            cache.clear();
            memoized_arrangements(&mut cache, spring_conditions, 0_u8, &windows)
        })
        .sum()
}

pub fn solve2(input: Input) -> u64 {
    let repetitions = 5_usize;
    let mut cache = Cache::new();

    input
        .into_iter()
        .map(|(spring_conditions, windows)| {
            let mut s: Vec<u8> =
                Vec::with_capacity(spring_conditions.len() * repetitions + spring_conditions.len());
            let mut w: Vec<u8> = Vec::with_capacity(windows.len() * repetitions);
            for i in 0..repetitions {
                w.extend_from_slice(&windows);
                s.extend_from_slice(spring_conditions);

                if i < (repetitions - 1) {
                    s.push(b'?');
                }
            }

            cache.clear();
            memoized_arrangements(&mut cache, &s, 0_u8, &w)
        })
        .sum()
}

fn memoized_arrangements<'b>(
    cache: &mut Cache,
    conditions: &'b [u8],
    cur_win_len: u8,
    windows: &'b [u8],
) -> u64 {
    if conditions.is_empty() {
        let superfluous_windows = windows.len();

        if cur_win_len == 0 && superfluous_windows == 0 {
            return 1;
        }

        if superfluous_windows == 1 && cur_win_len == windows[0] {
            return 1;
        }

        return 0;
    }

    // invalid state
    if cur_win_len > 0 && windows.is_empty() {
        return 0;
    }

    // memoize fn parameters (conditions, cur_win_len, windows)
    let cache_key = (conditions.len() as u8, cur_win_len, windows.len() as u8);
    let next_cond = &conditions[1..];

    match cache.get(&cache_key) {
        Some(&arrangements) => arrangements,
        None => {
            let arrangements = match (conditions[0], cur_win_len) {
                // early exit; invalid window size
                (b'.', _) if cur_win_len > 0 && cur_win_len != windows[0] => 0,

                // continue (no window)
                (b'.', 0) => {
                    if let Some((offset, _)) = conditions.iter().find_position(|&&p| p != b'.') {
                        memoized_arrangements(cache, &conditions[offset..], 0, windows)
                    } else {
                        memoized_arrangements(cache, &[], 0, windows)
                    }
                }

                // close window
                (b'.', _) => memoized_arrangements(cache, next_cond, 0, &windows[1..]),

                // open window
                (b'#', 0) => memoized_arrangements(cache, next_cond, 1, windows),

                // continue (in window)
                (b'#', _) => {
                    if let Some((offset, _)) = conditions.iter().find_position(|&&p| p != b'#') {
                        memoized_arrangements(
                            cache,
                            &conditions[offset..],
                            cur_win_len + offset as u8,
                            windows,
                        )
                    } else {
                        memoized_arrangements(
                            cache,
                            &[],
                            cur_win_len + conditions.len() as u8,
                            windows,
                        )
                    }
                }

                // branch
                (b'?', 0) => {
                    memoized_arrangements(cache, next_cond, 1, windows)
                        + memoized_arrangements(cache, next_cond, 0, windows)
                }

                (b'?', _) => {
                    let take = memoized_arrangements(cache, next_cond, cur_win_len + 1, windows);
                    let next = if cur_win_len == windows[0] {
                        memoized_arrangements(cache, next_cond, 0, &windows[1..])
                    } else {
                        0
                    };

                    take + next
                }

                _ => unreachable!("unknown condition symbol!"),
            };

            cache.insert(cache_key, arrangements);
            arrangements
        }
    }
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Vec<(&'s [u8], Vec<u8>)>> {
    let mut values: Vec<(&'s [u8], Vec<u8>)> = Vec::with_capacity(1000);

    while let Some(value) = opt(terminated(parse_line, newline)).parse_next(input)? {
        values.push(value);
    }

    Ok(values)
}

fn parse_line<'s>(input: &mut &'s str) -> PResult<(&'s [u8], Vec<u8>)> {
    let mut values: Vec<u8> = Vec::with_capacity(8);

    let vents = take_until0(" ").parse_next(input)?;
    let _ = tag(" ").parse_next(input)?;

    while let Some(value) = opt(terminated(parse_u32, ',')).parse_next(input)? {
        values.push(value as u8);
    }

    values.push(parse_u32(input)? as u8);

    Ok((vents.as_bytes(), values))
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day12::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 21)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(12, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 525152)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(12, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
