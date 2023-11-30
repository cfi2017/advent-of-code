use lazy_static::lazy_static;
use crate::utils::input;
use std::sync::Mutex;
use std::collections::HashMap;

pub trait Puzzle<I, R1, R2, const Y: i32, const D: i32>: Send
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


lazy_static! {
    static ref PUZZLE_REGISTRY: Mutex<HashMap<(i32, i32), Box<dyn AnyPuzzle>>> = Mutex::new(HashMap::new());
}

pub trait AnyPuzzle: Send {
    fn run(&self);

    fn year_day(&self) -> (i32, i32);
}

impl<I, R1, R2, const Y: i32, const D: i32> AnyPuzzle for dyn Puzzle<I, R1, R2, Y, D>
    where I: Clone,
          R1: std::fmt::Display,
          R2: std::fmt::Display,
{
    fn run(&self) {
        self.run();
    }

    fn year_day(&self) -> (i32, i32) {
        self.year_day()
    }

}

#[macro_export]
macro_rules! aoc_day {
    (year = $year:expr, day = $day:expr,
     fn sanitize_input($input:ident : &str) -> $input_type:ty {
        $($sanitize_body:tt)*
    }
    fn part_one($part_one_input:ident : $input_type_a:ty) -> $part_one_output:ty {
        $($part_one_body:tt)*
    }
    fn part_two($part_two_input:ident : $input_type_b:ty) -> $part_two_output:ty {
        $($part_two_body:tt)*
    }) => {
        struct PuzzleDay;

        impl Puzzle<$input_type, $part_one_output, (), $year, $day> for PuzzleDay {
            fn sanitize_input(&self, $input: &str) -> $input_type {
                $($sanitize_body)*
            }

            fn solve_a(&self, $part_one_input: $input_type_a) -> $part_one_output {
                $($part_one_body)*
            }

            fn solve_b(&self, $part_two_input: $input_type_b) -> $part_two_output {
                $($part_two_body)*
            }
        }
        {
            use std::any::Any;
            let mut registry = PUZZLE_REGISTRY.lock().unwrap();
            registry.insert(($year, $day), Box::new(PuzzleDay) as Box<dyn AnyPuzzle>);
        }
    };
}

pub fn run_year(year: i32) {
    let registry = PUZZLE_REGISTRY.lock().unwrap();
    for (&(puzzle_year, _), puzzle) in registry.iter() {
        if puzzle_year == year {
            puzzle.run();
        }
    }
}

pub fn run_day(year: i32, day: i32) {
    let registry = PUZZLE_REGISTRY.lock().unwrap();
    if let Some(puzzle) = registry.get(&(year, day)) {
        puzzle.run();
    } else {
        println!("Puzzle for Year {}, Day {} not found.", year, day);
    }
}


