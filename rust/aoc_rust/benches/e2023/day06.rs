use criterion::{black_box, criterion_group, Criterion};


use aoc_rust::e2023::day06;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = day06::get_input();
    // let input = day01::sanitize_input(&input);
    c.benchmark_group("2023_06")
        .throughput(criterion::Throughput::Bytes(input.len() as u64))
        .bench_function("solve_a", |b| b.iter(|| day06::solve_a(black_box(day06::sanitize_a(&input)))))
        .bench_function("solve_b", |b| b.iter(|| day06::solve_b(black_box(day06::sanitize_b(&input)))));
}

criterion_group!(benches, solve_benchmark);

