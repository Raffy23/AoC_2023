use aoc_2023::{day02, utils::read_input, utils::Part::Part1, day01};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, day01_benchmark, day02_benchmark);
criterion_main!(benches);

fn day01_benchmark(c: &mut Criterion) {
    let input = read_input(1, Part1).expect("Unable to read input file!");
    let lines = day01::parse_input(&input);

    c.bench_function("Day01 input parsing", |b| {
        b.iter(|| day01::parse_input(black_box(&input)))
    });

    c.bench_function("Day01 Part1", |b| {
        b.iter(|| day01::solve1(black_box(lines.clone())))
    });

    c.bench_function("Day01 Part2", |b| {
        b.iter(|| day01::solve2(black_box(lines.clone())))
    });
}

fn day02_benchmark(c: &mut Criterion) {
    let input = read_input(2, Part1).expect("Unable to read input file!");
    let (_, games) = day02::parse_input(&input).expect("Unable to parse input!");

    c.bench_function("Day02 input parsing", |b| {
        b.iter(|| day02::parse_input(black_box(&input)))
    });

    c.bench_function("Day02 Part1", |b| {
        b.iter(|| day02::solve1(black_box(games.clone())))
    });

    c.bench_function("Day02 Part2", |b| {
        b.iter(|| day02::solve2(black_box(games.clone())))
    });
}
