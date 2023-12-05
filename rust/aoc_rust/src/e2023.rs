use crate::aoc::Puzzle;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
mod day05;

pub fn run() {
    day02::run(true);
    day03::run(true);
    day04::run(true);
    day01::run(true);
}

pub fn run_day(day: u32, display: bool) {
    match day {
        1 => day01::run(display),
        2 => day02::run(display),
        3 => day03::run(display),
        4 => day04::run(display),
        _ => println!("Day {} not implemented yet.", day),
    }
}

