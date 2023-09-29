use criterion::criterion_main;
mod e2019;
mod e2018;

criterion_main! {
    // 2018
    e2018::day01::benches,

    // 2019
    e2019::day01::benches,
    e2019::day04::benches,
    e2019::day12::benches,
}
