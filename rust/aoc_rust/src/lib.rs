#![feature(concat_idents)]
#![feature(anonymous_lifetime_in_impl_trait)]

use aoc_runner_derive::aoc_lib;
use clap::Parser;

pub mod e2018;
pub mod e2019;
pub mod e2021;
pub mod e2022;
pub mod e2023;
pub mod aoc;

pub mod utils;

aoc_lib! {year = 2022}
macro_rules! run_year {
    ($year:ident) => {
        {
            println!("========{}========", "$year");
            $year::run();
        }
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: Option<u32>,
    #[arg(short, long)]
    day: Option<u32>,
    #[arg(short, long)]
    iterations: Option<u32>,
    #[arg(short, long, default_value_t = false)]
    no_print: bool,
}

pub fn run() {
    let args = Args::parse();
    if let Some(year) = args.year {
        if let Some(day) = args.day {
            if let Some(iterations) = args.iterations {
                for _ in 0..iterations {
                    run_day(year, day, !args.no_print);
                }
            } else {
                run_day(year, day, !args.no_print);
            }
        } else {
            run_y(year as i32);
        }
    } else {
        run_all();
    }
}

pub fn run_all() {
    println!("Running all puzzles.");
    run_year!(e2018);
    run_year!(e2019);
    run_year!(e2021);
    run_year!(e2022);
    run_year!(e2023);
}

pub fn run_y(year: i32) {
    match year {
        2018 => run_year!(e2018),
        2019 => run_year!(e2019),
        2021 => run_year!(e2021),
        2022 => run_year!(e2022),
        2023 => run_year!(e2023),
        _ => println!("Year {} not implemented yet.", year),
    }
}

pub fn run_day(year: u32, day: u32, display: bool) {
    match year {
        2023 => e2023::run_day(day, display),
        _ => println!("Year {} not implemented yet.", year),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

