use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::aoc::Puzzle;
use aoc_rust::e2019::day01::PuzzleDay;

pub fn solve_benchmark(c: &mut Criterion) {
    let masses = PuzzleDay.get_input();
    let masses = PuzzleDay.sanitize_input(&masses);
    c.bench_function("2019_01_solve_a", |b| b.iter(|| PuzzleDay.solve_a(black_box(masses.clone()))));
    c.bench_function("2019_01_solve_b", |b| b.iter(|| PuzzleDay.solve_b(black_box(masses.clone()))));
}

criterion_group!(benches, solve_benchmark);

