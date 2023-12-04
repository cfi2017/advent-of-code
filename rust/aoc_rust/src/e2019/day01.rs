use crate::aoc::Puzzle;


pub fn fuel_for(i: &i32) -> i32 {
    i / 3 - 2
}

pub fn recursive_fuel_for(i: &i32) -> i32 {
    let mut added = fuel_for(i);
    let mut sum = 0;
    while added > 0 {
        sum += added;
        added = fuel_for(&added);
    }
    return sum;
}
pub struct PuzzleDay;
impl Puzzle<Vec<i32>, i32, i32, 2019, 1> for PuzzleDay {
    fn sanitize_input(&self, input: &str) -> Vec<i32> {
        input.split("\n")
            .filter(|x| !x.is_empty())
            .map(str::parse::<i32>)
            .map(Result::unwrap).collect()
    }

    fn solve_a(&self, mut input: Vec<i32>) -> i32 {
        input
            .iter()
            .map(fuel_for)
            .reduce(|sum, x| sum + x)
            .unwrap()
    }
    fn solve_b(&self, mut input: Vec<i32>) -> i32 {
        input
            .iter()
            .map(recursive_fuel_for)
            .reduce(|sum, x| sum + x)
            .unwrap()
    }
}
