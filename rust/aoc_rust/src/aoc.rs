use std::any::TypeId;
use lazy_static::lazy_static;
use crate::utils::input;
use std::sync::Mutex;
use std::collections::HashMap;

pub trait Puzzle<I, R1, R2, const Y: i32, const D: i32>: Send + Sync
    where I: Clone,
          R1: std::fmt::Display,
          R2: std::fmt::Display,
{
    fn year_day(&self) -> (i32, i32) {
        (Y, D)
    }
    fn get_input(&self) -> String {
        input::read_input(Y, D).unwrap()
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

// add_test macro
#[macro_export]
macro_rules! add_test {
    ($name:ident, $part:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            let input = PuzzleDay.sanitize_input(input);
            assert_eq!(PuzzleDay.$part(input.clone()), $expected);
        }
    };
}
