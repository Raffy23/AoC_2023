use aoc_2023::{day01, day02, utils::read_input, utils::Part::Part1, day03, day04, day05, day07, day06, day08, day09, day10, day11, day12, day13, day15, day14, day16, };
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    day01_benchmark,
    day02_benchmark,
    day03_benchmark,
    day04_benchmark,
    day05_benchmark,
    day06_benchmark,
    day07_benchmark,
    day08_benchmark,
    day09_benchmark,
    day10_benchmark,
    day11_benchmark,
    day12_benchmark,
    day13_benchmark,
    day14_benchmark,
    day15_benchmark,
    day16_benchmark
);
criterion_main!(benches);

fn day01_benchmark(c: &mut Criterion) {
    let input = read_input(1, Part1).expect("Unable to read input file!");

    c.bench_function("Day01 input parsing", |b| {
        b.iter(|| day01::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day01 Part1", |b| {
        b.iter(|| day01::solve1(day01::parse_input(black_box(&mut input.as_str()))))
    });

    c.bench_function("Day01 Part2", |b| {
        b.iter(|| day01::solve2(day01::parse_input(black_box(&mut input.as_str()))))
    });
}

fn day02_benchmark(c: &mut Criterion) {
    let input = read_input(2, Part1).expect("Unable to read input file!");

    c.bench_function("Day02 input parsing", |b| {
        b.iter(|| day02::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day02 Part1", |b| {
        b.iter(|| day02::solve1(day02::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day02 Part2", |b| {
        b.iter(|| day02::solve2(day02::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}


fn day03_benchmark(c: &mut Criterion) {
    let input = read_input(3, Part1).expect("Unable to read input file!");

    c.bench_function("Day03 input parsing", |b| {
        b.iter(|| day03::parse_input(black_box(&input)))
    });

    c.bench_function("Day03 Part1", |b| {
        b.iter(|| day03::solve1(day03::parse_input(black_box(&input))))
    });

    c.bench_function("Day03 Part2", |b| {
        b.iter(|| day03::solve2(day03::parse_input(black_box(&input))))
    });
}

fn day04_benchmark(c: &mut Criterion) {
    let input = read_input(4, Part1).expect("Unable to read input file!");

    c.bench_function("Day04 input parsing", |b| {
        b.iter(|| day04::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day04 Part1", |b| {
        b.iter(|| day04::solve1(day04::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day04 Part2", |b| {
        b.iter(|| day04::solve2(day04::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day05_benchmark(c: &mut Criterion) {
    let input = read_input(5, Part1).expect("Unable to read input file!");

    c.bench_function("Day05 input parsing", |b| {
        b.iter(|| day05::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day05 Part1", |b| {
        b.iter(|| day05::solve1(day05::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day05 Part2", |b| {
        b.iter(|| day05::solve2(day05::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day06_benchmark(c: &mut Criterion) {
    let input = read_input(6, Part1).expect("Unable to read input file!");

    c.bench_function("Day06 input parsing", |b| {
        b.iter(|| day06::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day06 Part1", |b| {
        b.iter(|| day06::solve1(day06::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day06 Part2", |b| {
        b.iter(|| day06::solve2(day06::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day07_benchmark(c: &mut Criterion) {
    let input = read_input(7, Part1).expect("Unable to read input file!");

    c.bench_function("Day07 input parsing", |b| {
        b.iter(|| day07::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day07 Part1", |b| {
        b.iter(|| day07::solve1(day07::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day07 Part2", |b| {
        b.iter(|| day07::solve2(day07::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day08_benchmark(c: &mut Criterion) {
    let input = read_input(8, Part1).expect("Unable to read input file!");

    c.bench_function("Day08 input parsing", |b| {
        b.iter(|| day08::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day08 Part1", |b| {
        b.iter(|| day08::solve1(day08::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day08 Part2", |b| {
        b.iter(|| day08::solve2(day08::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day09_benchmark(c: &mut Criterion) {
    let input = read_input(9, Part1).expect("Unable to read input file!");

    c.bench_function("Day09 input parsing", |b| {
        b.iter(|| day09::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day09 Part1", |b| {
        b.iter(|| day09::solve1(day09::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day09 Part2", |b| {
        b.iter(|| day09::solve2(day09::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day10_benchmark(c: &mut Criterion) {
    let input = read_input(10, Part1).expect("Unable to read input file!");

    c.bench_function("Day10 input parsing", |b| {
        b.iter(|| day10::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day10 Part1", |b| {
        b.iter(|| day10::solve1(day10::parse_input(black_box(&input))))
    });

    c.bench_function("Day10 Part2", |b| {
        b.iter(|| day10::solve2(day10::parse_input(black_box(&input))))
    });
}

fn day11_benchmark(c: &mut Criterion) {
    let input = read_input(11, Part1).expect("Unable to read input file!");

    c.bench_function("Day11 input parsing", |b| {
        b.iter(|| day11::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day11 Part1", |b| {
        b.iter(|| day11::solve1(day11::parse_input(black_box(&input))))
    });

    c.bench_function("Day11 Part2", |b| {
        b.iter(|| day11::solve2(day11::parse_input(black_box(&input))))
    });
}

fn day12_benchmark(c: &mut Criterion) {
    let input = read_input(12, Part1).expect("Unable to read input file!");

    c.bench_function("Day12 input parsing", |b| {
        b.iter(|| day12::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day12 Part1", |b| {
        b.iter(|| day12::solve1(day12::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day12 Part2", |b| {
        b.iter(|| day12::solve2(day12::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day13_benchmark(c: &mut Criterion) {
    let input = read_input(13, Part1).expect("Unable to read input file!");

    c.bench_function("Day13 input parsing", |b| {
        b.iter(|| day13::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day13 Part1", |b| {
        b.iter(|| day13::solve1(day13::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day13 Part2", |b| {
        b.iter(|| day13::solve2(day13::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day14_benchmark(c: &mut Criterion) {
    let input = read_input(14, Part1).expect("Unable to read input file!");

    c.bench_function("Day14 input parsing", |b| {
        b.iter(|| day14::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day14 Part1", |b| {
        b.iter(|| day14::solve1(day14::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day14 Part2", |b| {
        b.iter(|| day14::solve2(day14::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}

fn day15_benchmark(c: &mut Criterion) {
    let input = read_input(15, Part1).expect("Unable to read input file!");

    c.bench_function("Day15 input parsing", |b| {
        b.iter(|| day15::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day15 Part1", |b| {
        b.iter(|| day15::solve1(day15::parse_input(black_box(input.as_str()))))
    });

    c.bench_function("Day15 Part2", |b| {
        b.iter(|| day15::solve2(day15::parse_input(black_box(input.as_str()))))
    });
}

fn day16_benchmark(c: &mut Criterion) {
    let input = read_input(16, Part1).expect("Unable to read input file!");

    c.bench_function("Day16 input parsing", |b| {
        b.iter(|| day16::parse_input(black_box(&mut input.as_str())))
    });

    c.bench_function("Day16 Part1", |b| {
        b.iter(|| day16::solve1(day16::parse_input(black_box(&mut input.as_str())).unwrap()))
    });

    c.bench_function("Day16 Part2", |b| {
        b.iter(|| day16::solve2(day16::parse_input(black_box(&mut input.as_str())).unwrap()))
    });
}