use std::collections::HashSet;
use crate::aoc::Puzzle;

pub struct Day01;

impl Puzzle<Vec<i32>, i32, i32, 2018, 1> for Day01 {
    fn sanitize_input(&self, input: &str) -> Vec<i32> {
        input.split("\n")
            .filter(|s| !s.is_empty())
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect()
    }

    fn solve_a(&self, input: Vec<i32>) -> i32 {
        input.iter().fold(0, |a, b| a + *b)
    }

    fn solve_b(&self, input: Vec<i32>) -> i32 {
        let mut results = HashSet::new();
        let mut sum = 0;
        loop {
            for i in &input {
                sum += i;
                if results.contains(&sum) {
                    return sum
                }
                results.insert(sum);
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = vec![1, -2, 3, 1];

        assert_eq!(Day01.solve_a(input.clone()), 3);
        assert_eq!(Day01.solve_b(input.clone()), 2);
    }
}
