#![feature(concat_idents)]

use aoc_runner_derive::aoc_lib;
pub mod e2018;
pub mod e2019;
pub mod e2021;
pub mod e2022;
pub mod e2023;
pub mod aoc;

mod playground;
pub mod utils;

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! {year = 2022}
macro_rules! run_year {
    ($year:expr) => {
        println!("========{}========", "$year");
        let module_name = concat_idents!($year);
        $year::run();
    };
}

pub fn run() {
    println!("Running all puzzles.");
    e2018::run();
    e2019::run();
    e2021::run();
    e2022::run();
    // run_year!(e2018);
    // run_year!(e2019);
    // run_year!(e2021);
    // run_year!(e2022);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

