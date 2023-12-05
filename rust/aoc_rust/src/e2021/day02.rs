use std::str::FromStr;

use crate::aoc::Puzzle;
use crate::e2021::day02::submarine::{AdvancedSubmarine, SimpleSubmarine, Submarine};

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        let command = parts.next().ok_or(String::from("error splitting input values"))?;
        let value = parts.next()
            .ok_or(String::from("error splitting input values"))?;
        let value: i32 = str::parse(value).map_err(|_| String::from("error parsing value"))?;
        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(String::from("invalid command"))
        }
    }
}

pub struct Day02 {}

mod submarine {
    use crate::e2021::day02::Command;

    pub trait Submarine {
        fn command(&mut self, command: Command);
    }

    #[derive(Debug)]
    pub struct SimpleSubmarine {
        position: i32,
        depth: i32,
    }

    #[derive(Debug)]
    pub struct AdvancedSubmarine {
        position: i32,
        depth: i32,
        aim: i32,
    }

    impl AdvancedSubmarine {
        pub fn new() -> Self {
            AdvancedSubmarine {
                position: 0,
                depth: 0,
                aim: 0,
            }
        }

        pub fn position(&self) -> i32 {
            self.position
        }

        pub fn depth(&self) -> i32 {
            self.depth
        }
    }

    impl SimpleSubmarine {
        pub fn position(&self) -> i32 {
            self.position
        }

        pub fn depth(&self) -> i32 {
            self.depth
        }

        pub fn new() -> Self {
            SimpleSubmarine {
                position: 0,
                depth: 0,
            }
        }
    }

    impl Submarine for SimpleSubmarine {

        fn command(&mut self, command: Command) {
            match command {
                Command::Forward(val) => self.position += val,
                Command::Up(val) => self.depth -= val,
                Command::Down(val) => self.depth += val,
            };
        }
    }

    impl Submarine for AdvancedSubmarine {
        fn command(&mut self, command: Command) {
            match command {
                Command::Forward(val) => {
                    self.position += val;
                    self.depth += self.aim * val;
                },
                Command::Up(val) => self.aim -= val,
                Command::Down(val) => self.aim += val,
            }
        }
    }
}

impl Puzzle<Vec<Command>, i32, i32, 2021, 2> for Day02 {

    fn sanitize_input(&self, input: &str) -> Vec<Command> {
        input.split('\n').filter(|s| !s.is_empty())
            .map(str::parse::<Command>)
            .map(Result::unwrap)
            .collect()
    }

    fn solve_a(&self, input: Vec<Command>) -> i32 {
        let mut submarine = SimpleSubmarine::new();
        for command in input {
            submarine.command(command);
        }

        submarine.position() * submarine.depth()
    }

    fn solve_b(&self, input: Vec<Command>) -> i32 {
        let mut submarine = AdvancedSubmarine::new();
        for command in input {
            submarine.command(command);
        }
        submarine.position() * submarine.depth()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let puzzle = Day02{};
        let result_a = puzzle.solve_a(commands.clone());
        assert_eq!(result_a, 150);
        let result_b = puzzle.solve_b(commands.clone());
        assert_eq!(result_b, 900);
    }
}
