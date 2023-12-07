

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
mod day07;

pub fn run() {
    day01::run(true);
    day02::run(true);
    day03::run(true);
    day04::run(true);
    day05::run(true);
}

pub fn run_day(day: u32, display: bool) {
    match day {
        1 => day01::run(display),
        2 => day02::run(display),
        3 => day03::run(display),
        4 => day04::run(display),
        5 => day05::run(display),
        _ => println!("Day {} not implemented yet.", day),
    }
}

