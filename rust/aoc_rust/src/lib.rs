use aoc_runner_derive::aoc_lib;

pub mod e2018;
pub mod e2019;
pub mod e2021;
pub mod e2022;
pub mod aoc;

mod playground;
pub mod registry;

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! {year = 2022}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

