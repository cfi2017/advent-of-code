
use itertools::Itertools;
use crate::aoc::Puzzle;
use crate::e2019::state_machine::{QueueIO, StateMachine};

pub struct Day07;

const MAX_PHASE_INT: i32 = 3125; // 5 = 7775

impl Puzzle<StateMachine, i64, i64, 2019, 7> for Day07 {
    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine::from(input)
    }

    fn solve_a(&self, input: StateMachine) -> i64 {
        let mut max = 0;
        let digits = [0, 1, 2, 3, 4];
        for phases in digits.iter().permutations(5) {
            let machine = input.clone();
            let mut value = 0;
            for phase in phases {
                let mut io = QueueIO::new();
                let mut machine = machine.clone();
                // push in reverse order because we pop from the end
                io.input.push_back(*phase);
                io.input.push_back(value);
                machine.run(&mut io);
                value = io.output.pop().unwrap();
            }
            if value > max {
                max = value;
            }
        }
        max
    }

    fn solve_b(&self, input: StateMachine) -> i64 {
        let mut max = 0;
        let digits = [5, 6, 7, 8, 9];
        for phases in digits.iter().permutations(5) {
            //println!("phases: {:?}", phases);
            let mut machines = Vec::new();
            for phase in phases {
                let machine = input.clone();
                let mut io = QueueIO::new();
                io.input.push_back(*phase);
                machines.push((machine, io));
            }
            let mut value = 0;
            let mut i = 0;
            loop {
                let (machine, io) = &mut machines[i];
                io.input.push_back(value);
                machine.run_until_output_or_complete(io);
                // get last output without popping it
                value = *io.output.last().unwrap();
                if machine.is_done() && i == 4 {
                    break;
                }
                i = (i + 1) % 5;
            }
            if value > max {
                max = value;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let machine = StateMachine::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(Day07.solve_a(machine), 43210);
        let machine = StateMachine::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        assert_eq!(Day07.solve_a(machine), 54321);
    }

    #[test]
    fn test_solve_b() {
        let machine = StateMachine::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        assert_eq!(Day07.solve_b(machine), 139629729);
        let machine = StateMachine::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        assert_eq!(Day07.solve_b(machine), 18216);
    }

}


