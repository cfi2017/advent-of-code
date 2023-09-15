use crate::aoc::{parse_ints, Puzzle, read_input};

pub struct Day02;

#[derive(Clone)]
struct StateMachine {
    state: Vec<i32>,
    position: usize,
    done: bool,
}

impl StateMachine {
    fn next(&mut self) {
        let command = self.state[self.position];
        if command == 99 {
            self.done = true;
            return
        }
        let a = self.state[self.state[self.position + 1] as usize];
        let b = self.state[self.state[self.position + 2] as usize];
        let y = self.state[self.position + 3] as usize;
        match command {
            1 => self.state[y] = a + b,
            2 => self.state[y] = a * b,
            _ => (),
        }
        self.position += 4;
    }
}

impl Puzzle<StateMachine, i32, i32> for Day02 {
    fn get_input(&self) -> String {
        read_input(2019, 2).unwrap()
    }

    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine {
            state: parse_ints(input, ","),
            position: 0,
            done: false,
        }
    }

    fn solve_a(&self, mut input: StateMachine) -> i32 {
        loop {
            input.next();
            if input.done {
                break
            }
        }
        input.state[0]
    }

    fn solve_b(&self, input: StateMachine) -> i32 {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut input = input.clone();
                input.state[1] = noun;
                input.state[2] = verb;
                if self.solve_a(input) == 19690720 {
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
        input.state[1] = 12;
        input.state[2] = 2;
        assert_eq!(Day02.solve_a(input.clone()), 3562624);
        assert_eq!(Day02.solve_b(input.clone()), 8298);
    }
}
