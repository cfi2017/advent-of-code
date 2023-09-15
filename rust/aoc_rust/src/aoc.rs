use std::fs::File;
use std::io::Read;

pub fn get_input_path(year: i32, day: i32) -> String {
    format!("assets/input/{}/day{:02}.txt", year, day)
}

pub fn read_input(year: i32, day: i32) -> std::io::Result<String> {
    let mut file = File::open(get_input_path(year, day))?;
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer)?;
    return Ok(buffer);
}

pub trait Puzzle<I, R1, R2>
    where I: Clone
{
    fn get_input(&self) -> String;
    fn sanitize_input(&self, input: &str) -> I;
    fn solve(&self, input: I) -> (R1, R2) {
        return (self.solve_a(input.clone()), self.solve_b(input.clone()))
    }
    fn solve_a(&self, input: I) -> R1;
    fn solve_b(&self, input: I) -> R2;
}

pub fn parse_ints(input: &str, separator: &str) -> Vec<i32> {
    input.trim().split(separator).map(str::parse::<i32>).map(Result::unwrap).collect()
}
