use std::collections::VecDeque;

use itertools::Itertools;

pub fn solve1(input: &str) -> usize {
    let mut result = 0_usize;
    let mut current_iteration = 0_u32;

    for &c in input.as_bytes() {
        match c {
            b'\n' => { /* nothing */ }
            b',' => {
                result += current_iteration as usize;
                current_iteration = 0;
            }
            _ => {
                current_iteration += c as u32;
                current_iteration *= 17;
                current_iteration %= 256;
            }
        }
    }

    result + current_iteration as usize
}

#[derive(Debug, Clone, Copy)]
struct Lens<'s> {
    label: &'s str,
    focal_len: u8,
}

pub fn solve2<'s>(input: &'s str) -> usize {
    let mut boxes: Vec<VecDeque<Lens<'s>>> = vec![VecDeque::with_capacity(64); 256];

    let mut iterator = input.as_bytes().iter().enumerate();

    let mut hash = 0_u32;
    let mut label_start = 0;

    while let Some((index, &c)) = iterator.next() {
        match c {
            b'\n' => { /* nothing */ }
            b',' => {
                hash = 0;
                label_start = index + 1;
            }
            b'=' => {
                let (_, &focal_len) = iterator.next().unwrap();
                let focal_len = focal_len - b'0';
                let lens = Lens {
                    label: &input[label_start..index],
                    focal_len,
                };

                let lens_box = &mut boxes[hash as usize];
                if let Some((position, _)) =
                    lens_box.iter().find_position(|l| l.label == lens.label)
                {
                    lens_box[position].focal_len = lens.focal_len;
                } else {
                    lens_box.push_back(lens);
                }
            }
            b'-' => {
                let label = &input[label_start..index];
                let lens_box = &mut boxes[hash as usize];
                if let Some((position, _)) = lens_box.iter().find_position(|l| l.label == label) {
                    lens_box.remove(position);
                }
            }
            _ => {
                hash += c as u32;
                hash *= 17;
                hash %= 256;
            }
        }
    }

    boxes.into_iter().enumerate().map(|(box_index, lens_box)| {
        let mut score = 0_usize;

        for (lens_index, lens) in lens_box.iter().enumerate() {
            score += (box_index + 1) * (lens_index + 1) * lens.focal_len as usize;
        }

        score
    }).sum()
}

#[inline]
pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::{
        day15::{parse_input, solve1, solve2},
        utils::{read_input, Part},
    };

    const EXAMPLE_INPUT: &'static str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(parse_input(EXAMPLE_INPUT)), 1320)
    }

    #[test]
    fn solve_part1() {
        let input = read_input(15, Part::Part1).expect("unable to read input file");
        println!("{}", solve1(parse_input(input.as_str())))
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(parse_input(&mut EXAMPLE_INPUT)), 145)
    }

    #[test]
    fn solve_part2() {
        let input = read_input(15, Part::Part1).expect("unable to read input file");
        println!("{}", solve2(parse_input(input.as_str())))
    }
}
