use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::aoc::Puzzle;
use aoc_rust::e2023::day01;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = day01::get_input();
    // let input = day01::sanitize_input(&input);
    c.benchmark_group("2023_01")
        .throughput(criterion::Throughput::Bytes(input.len() as u64))
        .bench_function("solve_a", |b| b.iter(|| day01::solve_a(black_box(day01::sanitize_input(&input)))))
        .bench_function("solve_b", |b| b.iter(|| day01::solve_b(black_box(day01::sanitize_input(&input)))));
}

criterion_group!(benches, solve_benchmark);

