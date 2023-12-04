use criterion::criterion_main;
mod e2019;
mod e2018;
mod e2023;

criterion_main! {
    // 2018
    e2018::day01::benches,

    // 2019
    e2019::day01::benches,
    e2019::day04::benches,
    e2019::day12::benches,

    // 2023
    e2023::day01::benches,
    e2023::day02::benches,
    e2023::day03::benches,
    e2023::day04::benches,
}
