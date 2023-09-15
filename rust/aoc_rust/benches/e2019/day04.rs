use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::aoc::Puzzle;
use aoc_rust::e2019::day04::Day04;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = 246515..=739105;
    let day = Day04;
    c.bench_function("2019_04_solve_a", |b| b.iter(|| day.solve_a(black_box(input.clone()))));
    c.bench_function("2019_04_solve_b", |b| b.iter(|| day.solve_b(black_box(input.clone()))));
}

criterion_group!(benches, solve_benchmark);

