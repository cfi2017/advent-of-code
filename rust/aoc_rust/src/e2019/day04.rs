use std::ops::RangeInclusive;
// use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use crate::aoc::{Puzzle, read_input};

pub struct Day04;

impl Puzzle<RangeInclusive<i32>, i32, i32, 2019, 4> for Day04 {

    fn sanitize_input(&self, _: &str) -> RangeInclusive<i32> {
        246515..=739105
    }

    fn solve_a(&self, input: RangeInclusive<i32>) -> i32 {
        let mut counter = 0;
        for x in input.map(|x| x.to_string()) {
            if x.as_bytes().windows(2).fold_while(false, |r, b| {
                if b[0] <= b[1] {
                    return itertools::FoldWhile::Done(false);
                } else {
                    itertools::FoldWhile::Continue(r || b[0] == b[1])
                }
            }).into_inner() {
                counter += 1
            }
        }
        counter
    }



    fn solve_b(&self, input: RangeInclusive<i32>) -> i32 {
        let mut counter = 0;
        for x in input {
            let s = x.to_string();
            let bytes = s.as_bytes();
            if (
                bytes.windows(4).any(|b| b[0] != b[1] && b[2] != b[3] && b[1] == b[2])
                || check_bounds(bytes)
            )
                && bytes.windows(2).all(|b| b[0] <= b[1]) {
                counter += 1;
            }
        }
        counter
    }
}


fn check_bounds(b: &[u8]) -> bool {
    let len = b.len();
    b[0] == b[1] && b[1] != b[2] ||
    b[len - 1] == b[len - 2] && b[len - 2] != b[len - 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let day = Day04;
        let input = day.sanitize_input("");
        let a = day.solve_a(input.clone());
        let b = day.solve_b(input);
        println!("{}, {}", a, b)

    }
}
