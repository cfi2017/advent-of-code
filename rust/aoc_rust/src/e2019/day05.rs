

use crate::aoc::Puzzle;
use crate::e2019::state_machine::{QueueIO, StateMachine};

pub struct Day05;

impl Puzzle<StateMachine, i64, i64, 2019, 5> for Day05 {

    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine::from(input)
    }

    fn solve_a(&self, mut machine: StateMachine) -> i64 {
        let mut io = QueueIO::new();
        io.input.push_back(1);
        machine.run(&mut io);
        io.output.pop().unwrap()
    }

    fn solve_b(&self, input: StateMachine) -> i64 {
        let mut io = QueueIO::new();
        let mut machine = input;
        io.input.push_back(5);
        machine.run(&mut io);
        io.output.pop().unwrap()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {

    }
}
