use std::ops::Range;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};
use rangemap::RangeMap;

use crate::utils::{parse_u32, parse_u64};

type Input = (Vec<u64>, Vec<RangeMap<u64, (u64, u64)>>);

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

fn parse_triple_u32<'a>(input: &'a str) -> IResult<&'a str, (u64, u64, u64)> {
    let (input, fst) = parse_u32(input)?;
    let (input, snd) = preceded(tag(" "), parse_u32)(input)?;
    let (input, thd) = preceded(tag(" "), parse_u32)(input)?;

    Ok((input, (fst as u64, snd as u64, thd as u64)))
}

fn parse_map<'a>(
    name: &'static str,
    input: &'a str,
) -> IResult<&'a str, RangeMap<u64, (u64, u64)>> {
    let (input, _) = tag(name)(input)?;
    let (input, _) = newline(input)?;
    let (input, map) = terminated(separated_list1(newline, parse_triple_u32), newline)(input)?;
    let (input, _) = opt(newline)(input)?;

    let mut range_map = RangeMap::new();
    for (dst_start, src_start, src_len) in map {
        range_map.insert(src_start..(src_start + src_len), (dst_start, src_start));
    }

    Ok((input, range_map))
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = terminated(separated_list1(tag(" "), parse_u64), newline)(input)?;
    let (input, _) = newline(input)?;

    let (input, soil_map) = parse_map("seed-to-soil map:", input)?;
    let (input, fertilizer_map) = parse_map("soil-to-fertilizer map:", input)?;
    let (input, water_map) = parse_map("fertilizer-to-water map:", input)?;
    let (input, light_map) = parse_map("water-to-light map:", input)?;
    let (input, temperature_map) = parse_map("light-to-temperature map:", input)?;
    let (input, humidity_map) = parse_map("temperature-to-humidity map:", input)?;
    let (input, location_map) = parse_map("humidity-to-location map:", input)?;

    if input.len() > 0 {
        panic!("Could not fully parse input file");
    }

    let mut maps = Vec::with_capacity(7);
    maps.push(soil_map);
    maps.push(fertilizer_map);
    maps.push(water_map);
    maps.push(light_map);
    maps.push(temperature_map);
    maps.push(humidity_map);
    maps.push(location_map);

    Ok((input, (seeds, maps)))
}

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
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT).unwrap().1), 35)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(5, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&input).unwrap().1))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(EXAMPLE_INPUT).unwrap().1), 46)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(5, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&input).unwrap().1))
    }
}
