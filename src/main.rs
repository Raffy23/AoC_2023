use aoc_2023::{
    day02,
    utils::{read_input, Part}, day01,
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
        (2, 1) => println!("{}", day02::solve1(day02::parse_input(&input).unwrap().1)),
        (2, 2) => println!("{}", day02::solve2(day02::parse_input(&input).unwrap().1)),
        _ => todo!(),
    }
}
