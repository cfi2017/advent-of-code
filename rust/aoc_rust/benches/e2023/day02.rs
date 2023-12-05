use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::aoc::Puzzle;
use aoc_rust::e2023::day02;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = day02::get_input();
    let input = day02::sanitize_input(&input);
    c.bench_function("2023_02_solve_a", |b| b.iter(|| day02::solve_a(black_box(input.clone()))));
    c.bench_function("2023_02_solve_b", |b| b.iter(|| day02::solve_b(black_box(input.clone()))));
}

criterion_group!(benches, solve_benchmark);

