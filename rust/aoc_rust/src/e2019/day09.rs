use crate::aoc::Puzzle;
use crate::e2019::state_machine::StateMachine;

pub struct Day09;

impl Puzzle<StateMachine, i64, i64, 2019, 9> for Day09 {
    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine::from(input)
    }

    fn solve_a(&self, mut machine: StateMachine) -> i64 {
        machine.input.push_back(1);
        machine.run();
        machine.output.pop().unwrap()
    }

    fn solve_b(&self, mut machine: StateMachine) -> i64 {
        machine.input.push_back(2);
        machine.run();
        machine.output.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::e2019::state_machine::StateMachine;

    #[test]
    fn test_solve_a() {
        let mut machine = StateMachine::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        machine.run();
        assert_eq!(machine.output, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        let mut machine = StateMachine::from("1102,34915192,34915192,7,4,7,99,0");
        machine.run();
        println!("output: {:?}", machine.output);
        let mut machine = StateMachine::from("104,1125899906842624,99");
        machine.run();
        assert_eq!(machine.output, vec![1125899906842624]);
    }

}
