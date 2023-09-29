use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::aoc::Puzzle;
use aoc_rust::e2019::day12::Day12;

pub fn solve_benchmark(c: &mut Criterion) {
    let day = Day12;
    let input = day.get_input();
    let input = day.sanitize_input(&input);
    c.bench_function("2019_12_solve_a", |b| b.iter(|| day.solve_a(black_box(input.clone()))));
    c.bench_function("2019_12_solve_b", |b| b.iter(|| day.solve_b(black_box(input.clone()))));

    // c.bench_function("2019_12_gcd", |b| b.iter(|| aoc_rust::e2019::day12::gcd(black_box(12345678), black_box(87654321))));
    // c.bench_function("2019_12_gcd_loop", |b| b.iter(|| aoc_rust::e2019::day12::gcd_loop(black_box(12345678), black_box(87654321))));
}

criterion_group!(benches, solve_benchmark);

