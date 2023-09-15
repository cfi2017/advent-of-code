use criterion::{black_box, criterion_group, Criterion};
use std::fs::read_to_string;
use aoc_rust::e2019::day01::{solve, sanitise};

pub fn solve_benchmark(c: &mut Criterion) {
    let input = read_to_string("./assets/input/2019/day01.txt").unwrap();
    let masses = sanitise(input);
    c.bench_function("2019_01_solve", |b| b.iter(|| solve(black_box(masses.clone()))));
}

criterion_group!(benches, solve_benchmark);

