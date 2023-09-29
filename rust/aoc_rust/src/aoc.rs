use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use aoc_client::{AocClient, PuzzleDay};

pub fn get_input_path(year: i32, day: i32) -> String {
    format!("assets/input/{}/day{:02}.txt", year, day)
}

pub fn read_input(year: i32, day: i32) -> std::io::Result<String> {
    let mut file = File::open(get_input_path(year, day))?;
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub trait Puzzle<I, R1, R2, const Y: i32, const D: i32>
    where I: Clone,
          R1: std::fmt::Display,
          R2: std::fmt::Display,
{
    fn get_input(&self) -> String {
        read_input(Y, D).unwrap()
    }
    fn sanitize_input(&self, input: &str) -> I;
    fn solve(&self, input: I) -> (R1, R2) {
        (self.solve_a(input.clone()), self.solve_b(input.clone()))
    }

    fn run(&self) {
        let input = self.get_input();
        let input = self.sanitize_input(&input);
        let (a, b) = self.solve(input);
        // println!("---------------------------");
        println!("Year {}, Day {:02}: a = {:20} | b = {:20}", Y, D, a, b);
        // println!("a = {}", a);
        // println!("b = {}", b);
        // println!("---------------------------");
    }
    fn solve_a(&self, input: I) -> R1;
    fn solve_b(&self, input: I) -> R2;
}

pub fn parse_ints<T>(input: &str, separator: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
{
    input.trim().split(separator).map(str::parse::<T>).map(Result::unwrap).collect()
}
