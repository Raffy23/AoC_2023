use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use num::integer::lcm;
use winnow::{
    ascii::{alphanumeric1, newline},
    combinator::{opt, separated_pair, terminated},
    token::take_till,
    PResult, Parser,
};

type Input<'s> = (&'s str, HashMap<&'s str, u32>, Vec<(u32, u32)>);

pub fn solve1<'s>((directions, nodes, edges): Input<'s>) -> u64 {
    let start_node = *nodes.get("AAA").unwrap();
    let target_node = *nodes.get("ZZZ").unwrap();

    find_steps_between_node(
        &(directions, nodes, edges),
        start_node,
        &BTreeSet::from([target_node]),
    )
}

pub fn solve2<'s>((directions, nodes, edges): Input<'s>) -> u64 {
    let start_nodes = nodes
        .iter()
        .filter(|(name, _)| name.ends_with("A"))
        .map(|(_, value)| *value)
        .collect_vec();

    let target_nodes: BTreeSet<u32> = nodes
        .iter()
        .filter(|(name, _)| name.ends_with("Z"))
        .map(|(_, value)| *value)
        .collect();

    let input = (directions, nodes, edges);
    start_nodes
        .into_iter()
        .map(|start_node| find_steps_between_node(&input, start_node, &target_nodes))
        .fold(1, lcm)
}

fn find_steps_between_node<'s>(
    (directions, _, edges): &Input<'s>,
    start_node: u32,
    end_nodes: &BTreeSet<u32>,
) -> u64 {
    let mut current_node = start_node;
    let mut position = 0_u64;
    let mut directions = directions.as_bytes().into_iter().cycle();

    while !end_nodes.contains(&current_node) {
        position += 1;

        let next_node = if *directions.next().unwrap() == b'L' {
            edges[current_node as usize].0
        } else {
            edges[current_node as usize].1
        };

        current_node = next_node;
    }

    position
}

pub fn parse_input<'s>(input: &mut &'s str) -> PResult<Input<'s>> {
    let directions = take_till(0.., '\n').parse_next(input)?;
    let _ = newline(input)?;
    let _ = newline(input)?;

    let mut nodes = HashMap::new();
    let mut edges = Vec::with_capacity(772);

    let mut index = 0_u32;
    while let Some((name, left, right)) = opt(terminated(parse_row, newline)).parse_next(input)? {
        nodes.insert(name, index);
        edges.push((left, right));
        index += 1;
    }

    let nodes_data = edges
        .into_iter()
        .map(|(left, right)| (*nodes.get(left).unwrap(), *nodes.get(right).unwrap()))
        .collect_vec();

    Ok((directions, nodes, nodes_data))
}

fn parse_row<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str, &'s str)> {
    let name = terminated(alphanumeric1, " = (").parse_next(input)?;
    let (left, right) =
        terminated(separated_pair(alphanumeric1, ", ", alphanumeric1), ")").parse_next(input)?;

    Ok((name, left, right))
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day08::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT_1: &'static str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const EXAMPLE_INPUT_2: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const EXAMPLE_INPUT_3: &'static str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT_1).unwrap()), 2);
        assert_eq!(solve1(parse_input(&mut EXAMPLE_INPUT_2).unwrap()), 6);
    }

    #[test]
    fn solve_part1() {
        let input = read_input(8, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(&mut input.as_str()).unwrap()))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT_3).unwrap()), 6)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(8, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(&mut input.as_str()).unwrap()))
    }
}
