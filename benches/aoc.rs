use aoc_2023::{day01, day02, utils::read_input, utils::Part::Part1, day03, day04, day05, day07, day06, };
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    day01_benchmark,
    day02_benchmark,
    day03_benchmark,
    day04_benchmark,
    day05_benchmark,
    day06_benchmark,
    day07_benchmark
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