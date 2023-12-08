use std::collections::HashMap;

use itertools::Itertools;
use num::integer::lcm;
use winnow::{
    ascii::{alphanumeric1, newline},
    combinator::{opt, separated_pair, terminated},
    token::take_till,
    PResult, Parser,
};

type Node = u16;

type Input<'s> = (&'s str, HashMap<&'s str, Node>, Vec<(Node, Node, bool)>);

pub fn solve1<'s>((directions, node_lookup, mut nodes): Input<'s>) -> u64 {
    let start_node = *node_lookup.get("AAA").unwrap();
    let target_node = *node_lookup.get("ZZZ").unwrap();

    nodes[target_node as usize].2 = true;

    find_steps_to_end_node(&(directions, node_lookup, nodes), start_node)
}

pub fn solve2<'s>((directions, node_lookup, mut nodes): Input<'s>) -> u64 {
    let start_nodes = node_lookup
        .iter()
        .filter(|(name, _)| name.ends_with("A"))
        .map(|(_, value)| *value)
        .collect_vec();

    for (&name, &id) in node_lookup.iter() {
        if name.ends_with("Z") {
            nodes[id as usize].2 = true;
        }
    }

    let input = (directions, node_lookup, nodes);
    start_nodes
        .into_iter()
        .map(|start_node| find_steps_to_end_node(&input, start_node))
        .fold(1, lcm)
}

fn find_steps_to_end_node<'s>((directions, _, edges): &Input<'s>, start_node: Node) -> u64 {
    let mut current_node = start_node;
    let mut position = 0_u64;
    let mut directions = directions.as_bytes().into_iter().cycle();

    while !edges[current_node as usize].2 {
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

    let mut index = 0_u16;
    while let Some((name, left, right)) = opt(terminated(parse_row, newline)).parse_next(input)? {
        nodes.insert(name, index);
        edges.push((left, right));
        index += 1;
    }

    let nodes_data = edges
        .into_iter()
        .map(|(left, right)| (*nodes.get(left).unwrap(), *nodes.get(right).unwrap(), false))
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
