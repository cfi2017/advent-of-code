use crate::aoc::Puzzle;
use crate::e2019::state_machine::{QueueIO, StateMachine};

pub struct Day02;
impl Puzzle<StateMachine, i64, i64, 2019, 2> for Day02 {

    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine::from(input)
    }

    fn solve_a(&self, mut machine: StateMachine) -> i64 {
        machine.set_state(1, 12);
        machine.set_state(2, 2);
        machine.run(&mut QueueIO::new());
        machine.get_state(0)
    }

    fn solve_b(&self, machine: StateMachine) -> i64 {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut machine = machine.clone();
                machine.set_state(1, noun);
                machine.set_state(2, verb);
                machine.run(&mut QueueIO::new());
                if machine.get_state(0) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        -1
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let input = Day02.sanitize_input(input);
        assert_eq!(Day02.solve_a(input.clone()), 3500);
        let mut input = Day02.sanitize_input(&Day02.get_input());
        input.set_state(1, 12);
        input.set_state(2, 2);
        assert_eq!(Day02.solve_a(input.clone()), 3562624);
        assert_eq!(Day02.solve_b(input.clone()), 8298);
    }
}
