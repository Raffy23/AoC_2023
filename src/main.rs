use aoc_2023::{
    day02,
    utils::{read_input, Part}, day01, day03, day04, day05, day06, day07, day08, day10, day09,
};
use clap::Parser;

/// Simple runner for aoc 2023 days
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day that should be run (1 - 25)
    #[arg(short, long)]
    day: u8,

    /// Part that should be run (1,2)
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    run_day(args.day, args.part);
}

fn run_day(day: u8, part: u8) {
    let separate_input_files = vec![];

    let input = read_input(
        day,
        if separate_input_files.contains(&day) {
            match part {
                1 => Part::Part1,
                2 => Part::Part2,
                _ => panic!("Unknown part number!"),
            }
        } else {
            Part::Part1
        },
    )
    .expect("unable to read input file");

    print!("Day {:0>2} Part {}: ", day, part);
    
    match (day, part) {
        (1, 1) => println!("{}", day01::solve1(day01::parse_input(&input))),
        (1, 2) => println!("{}", day01::solve1(day01::parse_input(&input))),
        (2, 1) => println!("{}", day02::solve1(day02::parse_input(&mut input.as_str()).unwrap())),
        (2, 2) => println!("{}", day02::solve2(day02::parse_input(&mut input.as_str()).unwrap())),
        (3, 1) => println!("{}", day03::solve1(day03::parse_input(&input))),
        (3, 2) => println!("{}", day03::solve2(day03::parse_input(&input))),
        (4, 1) => println!("{}", day04::solve1(day04::parse_input(&mut input.as_str()).unwrap())),
        (4, 2) => println!("{}", day04::solve2(day04::parse_input(&mut input.as_str()).unwrap())),
        (5, 1) => println!("{}", day05::solve1(day05::parse_input(&mut input.as_str()).unwrap())),
        (5, 2) => println!("{}", day05::solve2(day05::parse_input(&mut input.as_str()).unwrap())),
        (6, 1) => println!("{}", day06::solve1(day06::parse_input(&mut input.as_str()).unwrap())),
        (6, 2) => println!("{}", day06::solve2(day06::parse_input(&mut input.as_str()).unwrap())),
        (7, 1) => println!("{}", day07::solve1(day07::parse_input(&mut input.as_str()).unwrap())),
        (7, 2) => println!("{}", day07::solve2(day07::parse_input(&mut input.as_str()).unwrap())),
        (8, 1) => println!("{}", day08::solve1(day08::parse_input(&mut input.as_str()).unwrap())),
        (8, 2) => println!("{}", day08::solve2(day08::parse_input(&mut input.as_str()).unwrap())),
        (9, 1) => println!("{}", day09::solve1(day09::parse_input(&mut input.as_str()).unwrap())),
        (9, 2) => println!("{}", day09::solve2(day09::parse_input(&mut input.as_str()).unwrap())),
        (10, 1) => println!("{}", day10::solve1(day10::parse_input(&input))),
        (10, 2) => println!("{}", day10::solve2(day10::parse_input(&input))),
        _ => todo!(),
    }
}
