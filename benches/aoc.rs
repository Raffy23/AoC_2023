use aoc_2023::{day01, day02, day03, utils::read_input, utils::Part::Part1};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, day01_benchmark, day02_benchmark, day03_benchmark);
criterion_main!(benches);

fn day01_benchmark(c: &mut Criterion) {
    let input = read_input(1, Part1).expect("Unable to read input file!");

    c.bench_function("Day01 input parsing", |b| {
        b.iter(|| day01::parse_input(black_box(&input)))
    });

    c.bench_function("Day01 Part1", |b| {
        b.iter(|| day01::solve1(day01::parse_input(black_box(&input))))
    });

    c.bench_function("Day01 Part2", |b| {
        b.iter(|| day01::solve2(day01::parse_input(black_box(&input))))
    });
}

fn day02_benchmark(c: &mut Criterion) {
    let input = read_input(2, Part1).expect("Unable to read input file!");

    c.bench_function("Day02 input parsing", |b| {
        b.iter(|| day02::parse_input(black_box(&input)))
    });

    c.bench_function("Day02 Part1", |b| {
        b.iter(|| day02::solve1(day02::parse_input(black_box(&input)).unwrap().1))
    });

    c.bench_function("Day02 Part2", |b| {
        b.iter(|| day02::solve2(day02::parse_input(black_box(&input)).unwrap().1))
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
