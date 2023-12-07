use std::ops::Range;

use itertools::Itertools;
use rangemap::RangeMap;
use winnow::{
    ascii::newline,
    combinator::{opt, preceded, separated, terminated},
    token::tag,
    PResult, Parser,
};

use crate::utils::{parse_u32, parse_u64};

type Input = (Vec<u64>, [RangeMap<u64, (u64, u64)>; 7]);

pub fn solve1((seeds, range_maps): Input) -> u64 {
    seeds
        .into_iter()
        .map(|seed| {
            range_maps.iter().fold(seed, |src, range_map| {
                let lookup_result = range_map.get(&src);

                if let Some(&(dst_start, src_start)) = lookup_result {
                    src - src_start + dst_start
                } else {
                    src
                }
            })
        })
        .min()
        .unwrap()
}

pub fn solve2((seeds, range_maps): Input) -> u64 {
    seeds
        .into_iter()
        .tuples()
        .map(|(seed_start, seed_len)| {
            range_maps
                .iter()
                .fold(
                    vec![seed_start..(seed_start + seed_len)],
                    |src_ranges, range_map| {
                        src_ranges
                            .into_iter()
                            .flat_map(|src_range| intersection(range_map, src_range))
                            .collect_vec()
                    },
                )
                .into_iter()
                .map(|range| range.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn intersection(range_map: &RangeMap<u64, (u64, u64)>, src_range: Range<u64>) -> Vec<Range<u64>> {
    let mut result = Vec::new();

    for (overlap_range, &(dst_start, src_start)) in range_map.overlapping(&src_range) {
        let start = src_range.start.max(overlap_range.start) - src_start + dst_start;
        let end = src_range.end.min(overlap_range.end) - src_start + dst_start;

        result.push(start..end);
    }

    for gap in range_map.gaps(&src_range) {
        result.push(gap);
    }

    result
}

fn parse_triple_u32<'s>(input: &mut &'s str) -> PResult<(u64, u64, u64)> {
    let fst = parse_u32(input)?;
    let snd = preceded(' ', parse_u32).parse_next(input)?;
    let thd = preceded(' ', parse_u32).parse_next(input)?;

    Ok((fst as u64, snd as u64, thd as u64))
}

fn parse_map<'s>(name: &'static str, input: &mut &'s str) -> PResult<RangeMap<u64, (u64, u64)>> {
    let _ = tag(name).parse_next(input)?;
    let _ = newline(input)?;

    let mut range_map = RangeMap::new();
    while let Some((dst_start, src_start, src_len)) =
        opt(terminated(parse_triple_u32, newline)).parse_next(input)?
    {
        range_map.insert(src_start..(src_start + src_len), (dst_start, src_start));
    }

    let _ = opt(newline).parse_next(input)?;

    Ok(range_map)
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Input> {
    let _ = tag("seeds: ").parse_next(input)?;
    let seeds: Vec<u64> = terminated(separated(0.., parse_u64, ' '), newline).parse_next(input)?;
    let _ = newline(input)?;

    let maps = [
        parse_map("seed-to-soil map:", input)?,
        parse_map("soil-to-fertilizer map:", input)?,
        parse_map("fertilizer-to-water map:", input)?,
        parse_map("water-to-light map:", input)?,
        parse_map("light-to-temperature map:", input)?,
        parse_map("temperature-to-humidity map:", input)?,
        parse_map("humidity-to-location map:", input)?
    ];

    if input.len() > 0 {
        panic!("Could not fully parse input file");
    }

    Ok((seeds, maps))
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day05::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT).unwrap()), 35)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(5, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT).unwrap()), 46)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(5, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
