use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::aoc::Puzzle;
use aoc_rust::e2023::day03;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = day03::get_input();
    let input = day03::sanitize_input(&input);
    c.bench_function("2023_03_solve_a", |b| b.iter(|| day03::solve_a(black_box(input.clone()))));
    c.bench_function("2023_03_solve_b", |b| b.iter(|| day03::solve_b(black_box(input.clone()))));
}

criterion_group!(benches, solve_benchmark);

