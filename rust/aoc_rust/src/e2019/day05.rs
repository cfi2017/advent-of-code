use crate::aoc::{parse_ints, Puzzle, read_input};

struct Day05;

enum Parameter {
    Direct(i32),
    Indirect(i32), // index, double deref
}

impl Parameter {
    fn new(direct: bool, value: i32) -> Self {
        if direct {
            Self::Direct(value)
        } else {
            Self::Indirect(value)
        }
    }

    fn resolve(self, state: &Vec<i32>) -> i32 {
        match self {
            Self::Direct(i) => state[i as usize],
            Self::Indirect(i) => state[state[i as usize] as usize]
        }
    }

    fn resolve_pos(self, state: &Vec<i32>) -> i32 {
        match self {
            Self::Direct(i) => i,
            Self::Indirect(i) => state[i as usize]
        }
    }
}

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JNZ(Parameter, Parameter),
    JZ(Parameter, Parameter),
    LT(Parameter, Parameter, Parameter),
    EQ(Parameter, Parameter, Parameter),
    Quit
}

struct ShittyBitflags(i32);

impl ShittyBitflags {
    fn pop(&mut self) -> bool {
        let result = self.0 % 10 == 1;
        self.0 /= 10;
        result
    }
}

impl From<(usize, i32)> for Instruction {
    fn from(data: (usize, i32)) -> Self {
        let position = data.0;
        let position = position as i32;
        let instruction_value = data.1;
        let opcode = instruction_value % 100;
        let mut bitflags = ShittyBitflags(instruction_value / 100);


        match opcode {
            1 => Instruction::Add(
                Parameter::new(bitflags.pop(), position + 1),
                Parameter::new(bitflags.pop(), position + 2),
                Parameter::new(bitflags.pop(), position + 3)
            ),
            2 => Instruction::Multiply(
                Parameter::new(bitflags.pop(), position + 1),
                Parameter::new(bitflags.pop(), position + 2),
                Parameter::new(bitflags.pop(), position + 3)
            ),
            3 => Instruction::Input(
                Parameter::new(bitflags.pop(), position + 1),
            ),
            4 => Instruction::Output(
                Parameter::new(bitflags.pop(), position + 1),
            ),
            5 => Instruction::JNZ(
                Parameter::new(bitflags.pop(), position + 1),
                Parameter::new(bitflags.pop(), position + 2),
            ),
            6 => Instruction::JZ(
                Parameter::new(bitflags.pop(), position + 1),
                Parameter::new(bitflags.pop(), position + 2),
            ),
            7 => Instruction::LT(
                Parameter::new(bitflags.pop(), position + 1),
                Parameter::new(bitflags.pop(), position + 2),
                Parameter::new(bitflags.pop(), position + 3)
            ),
            8 => Instruction::EQ(
                Parameter::new(bitflags.pop(), position + 1),
                Parameter::new(bitflags.pop(), position + 2),
                Parameter::new(bitflags.pop(), position + 3)
            ),
            99 => Instruction::Quit,
            _ => panic!("uh-oh")
        }
    }
}

#[derive(Clone)]
struct StateMachine {
    state: Vec<i32>,
    position: usize,
    done: bool,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl StateMachine {
    fn next(&mut self) {
        let command = Instruction::from((self.position, self.state[self.position]));
        match command {
            Instruction::Add(a, b, y) => {
                let y = y.resolve_pos(&self.state) as usize;
                self.state[y] = a.resolve(&self.state) + b.resolve(&self.state);
                self.position += 4;
            }
            Instruction::Multiply(a, b, y) => {
                let y = y.resolve_pos(&self.state) as usize;
                self.state[y] = a.resolve(&self.state) * b.resolve(&self.state);
                self.position += 4;
            }
            Instruction::Input(to) => {
                println!("trying to read input");
                let y = to.resolve_pos(&self.state) as usize;
                self.state[y] = self.input.pop().expect("looked for input where there was none");
                self.position += 2;
            }
            Instruction::Output(from) => {
                self.output.push(from.resolve(&self.state));
                self.position += 2;
            }
            Instruction::Quit => {
                self.done = true;
                self.position += 1;
            }
            Instruction::JNZ(t, p) => {
                if t.resolve(&self.state) != 0 {
                    self.position = p.resolve(&self.state) as usize;
                } else {
                    self.position += 3;
                }
            }
            Instruction::JZ(t, p) => {
                if t.resolve(&self.state) == 0 {
                    self.position = p.resolve(&self.state) as usize;
                } else {
                    self.position += 3;
                }
            }
            Instruction::LT(a, b, y) => {
                let y = y.resolve_pos(&self.state) as usize;
                if a.resolve(&self.state) < b.resolve(&self.state) {
                    self.state[y] = 1;
                } else {
                    self.state[y] = 0;
                }
                self.position += 4;
            }
            Instruction::EQ(a, b, y) => {
                let y = y.resolve_pos(&self.state) as usize;
                if a.resolve(&self.state) == b.resolve(&self.state) {
                    self.state[y] = 1;
                } else {
                    self.state[y] = 0;
                }
                self.position += 4;
            }
        }
    }
}

impl Puzzle<StateMachine, StateMachine, i32> for Day05 {
    fn get_input(&self) -> String {
        read_input(2019, 5).unwrap()
    }

    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine {
            state: parse_ints(input, ","),
            position: 0,
            done: false,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn solve_a(&self, mut machine: StateMachine) -> StateMachine {
        if machine.input.len() != 0 {
            println!("{:?}", machine.input);
        }
        loop {
            machine.next();
            if machine.done {
                break
            }
        }
        machine
    }

    fn solve_b(&self, input: StateMachine) -> i32 {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut input = input.clone();
                input.state[1] = noun;
                input.state[2] = verb;
                let machine = self.solve_a(input);

                if machine.state[0] == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        -1
    }
}



#[cfg(test)]
mod tests {
    use crate::e2019::day02::Day02;
    use super::*;

    #[test]
    fn test_solve() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let input = Day05.sanitize_input(input);
        let machine = Day05.solve_a(input.clone());
        assert_eq!(machine.state[0], 3500);
        let day02 = Day02{};
        let mut input = Day05.sanitize_input(&read_input(2019, 2).unwrap());
        input.state[1] = 12;
        input.state[2] = 2;
        let machine = Day05.solve_a(input.clone());
        assert_eq!(machine.state[0], 3562624);
        assert_eq!(Day05.solve_b(input.clone()), 8298);



        let input = Day05.sanitize_input(&Day05.get_input());
        let mut machine = input.clone();
        machine.input.push(1);
        let machine = Day05.solve_a(machine);
        println!("{:?}", machine.output);

        let mut machine = input.clone();
        machine.input.push(5);

        let machine = Day05.solve_a(machine);
        println!("{:?}", machine.output);

        let machine = Day05.sanitize_input("3,9,8,9,10,9,4,9,99,-1,8");
        let mut working = machine.clone();
        working.input.push(7);
        let working = Day05.solve_a(working);
        println!("{:?}", machine.output);
        let machine = Day05.sanitize_input("3,9,7,9,10,9,4,9,99,-1,8");
        let mut working = machine.clone();
        working.input.push(4);
        let working = Day05.solve_a(working);
        println!("{:?}", machine.output);
        let machine = Day05.sanitize_input("3,3,1107,-1,8,3,4,3,99");
        let mut working = machine.clone();
        working.input.push(4);
        let working = Day05.solve_a(working);
        println!("{:?}", machine.output);
        let machine = Day05.sanitize_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut working = machine.clone();
        working.input.push(4);
        let working = Day05.solve_a(working);
        println!("{:?}", machine.output);

    }
}
