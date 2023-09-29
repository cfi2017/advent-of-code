use crate::aoc::{parse_ints, Puzzle, read_input};

pub struct Day07;

impl Puzzle<Vec<i32>, i32, i32, 2021, 7> for Day07 {

    fn sanitize_input(&self, input: &str) -> Vec<i32> {
        parse_ints(input, ",")
    }

    fn solve_a(&self, mut input: Vec<i32>) -> i32 {
        input.sort();
        let mid = input.len() / 2;
        let median = input[mid];
        input.iter().map(|x| (*x - median).abs()).fold(0, |a, b| a + b)
    }

    fn solve_b(&self, input: Vec<i32>) -> i32 {
        let average = (input.iter().sum::<i32>() as f32 / input.len() as f32).round() as i32;
        input.iter().map(|x| (*x - average).abs()).fold(0, |a, b| a + (b * (b + 1) / 2))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let input = Day07.sanitize_input(input);
        assert_eq!(Day07.solve_a(input.clone()), 37);
        assert_eq!(Day07.solve_b(input.clone()), 168);
    }
}
