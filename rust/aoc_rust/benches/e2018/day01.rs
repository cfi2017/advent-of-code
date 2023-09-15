
use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::e2018::day01::Day01;
use aoc_rust::aoc::Puzzle;

pub fn solve_benchmark(c: &mut Criterion) {

    let input = Day01.get_input();
    let input = Day01.sanitize_input(input.as_str());
    c.bench_function("2018_01_solve_a", |b| b.iter(|| Day01.solve_a(black_box(input.clone()))));
    c.bench_function("2018_01_solve_b", |b| b.iter(|| Day01.solve_b(black_box(input.clone()))));
}

criterion_group!(benches, solve_benchmark);
