use criterion::{black_box, criterion_group, Criterion};


use aoc_rust::e2023::day05;

pub fn solve_benchmark(c: &mut Criterion) {
    let input = day05::get_input();
    // let input = day01::sanitize_input(&input);
    c.benchmark_group("2023_05")
        .throughput(criterion::Throughput::Bytes(input.len() as u64))
        .bench_function("solve_a", |b| b.iter(|| day05::solve_a(black_box(day05::sanitize_input_a(&input)))))
        .bench_function("solve_b", |b| b.iter(|| day05::solve_b(black_box(day05::sanitize_input_b(&input)))));
}

criterion_group!(benches, solve_benchmark);

