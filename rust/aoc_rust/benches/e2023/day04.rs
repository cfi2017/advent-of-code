use criterion::{black_box, criterion_group, Criterion};


use aoc_rust::e2023::day04;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = day04::get_input();
    let input = day04::sanitize_input(&input);
    c.bench_function("2023_04_solve_a", |b| b.iter(|| day04::solve_a(black_box(input.clone()))));
    c.bench_function("2023_04_solve_b", |b| b.iter(|| day04::solve_b(black_box(input.clone()))));
}

criterion_group!(benches, solve_benchmark);

