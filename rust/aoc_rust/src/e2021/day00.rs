#![cfg(target_os = "linux")]
use crate::aoc::Puzzle;

struct Day00;

impl Puzzle<Vec<String>, i32, i32> for Day00 {
    fn get_input(&self) -> String {
        unimplemented!()
    }

    fn sanitize_input(&self, _: &str) -> Vec<String> {
        unimplemented!()
    }

    fn solve_a(&self, _: Vec<String>) -> i32 {
        unimplemented!()
    }

    fn solve_b(&self, _: Vec<String>) -> i32 {
        unimplemented!()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        Day00.solve(vec![String::from("abc")]);
    }
}
