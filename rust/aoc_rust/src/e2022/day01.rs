use std::io::BufRead;
use itertools::Itertools;
use crate::aoc::{Puzzle, read_input};

struct Day01;

impl Puzzle<Vec<String>, i32, i32, 2022, 1> for Day01 {
    fn get_input(&self) -> String {
        read_input(2022, 1).unwrap()
    }

    fn sanitize_input(&self, input: &str) -> Vec<String> {
        input.split("\n\n").map(|x| x.trim().to_string()).collect()
    }

    fn solve_a(&self, elves: Vec<String>) -> i32 {
        elves.iter()
            .map(|elf: &String| elf
                .split("\n")
                .map(|item| str::parse::<i32>(item).unwrap())
                .sum::<i32>()
            ).sorted()
            .rev()
            .nth(0)
            .unwrap()
    }

    fn solve_b(&self, elves: Vec<String>) -> i32 {
        elves.iter()
            .map(|elf: &String| elf
                .split("\n")
                .map(|item| str::parse::<i32>(item).unwrap())
                .sum::<i32>()
            ).sorted()
            .rev()
            .take(3)
            .sum::<i32>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = Day01.get_input();
        let input = Day01.sanitize_input(&input);
        let result = Day01.solve_a(input);
        println!("{}", result)
    }
}
