use std::collections::VecDeque;
use std::fmt::{Display, Formatter};


use crate::utils::input::parse_ints;

enum ParameterMode {
    Immediate,
    Position, // index, double deref
    Relative // same as indirect, but offset with relative base
}

impl From<i64> for ParameterMode {
    fn from(i: i64) -> Self {
        match i {
            // aka indirect, deref mode
            0 => Self::Position,
            // aka direct mode
            1 => Self::Immediate,
            // aka relative mode
            2 => Self::Relative,
            _ => panic!("unknown parameter mode: {}", i)
        }
    }
}

struct Parameter {
    mode: ParameterMode,
    // position of the parameter on the tape
    arg_pos: i64
}

impl Parameter {
    fn new(mode: ParameterMode, value: i64) -> Self {
        Self {
            mode,
            arg_pos: value
        }
    }

    // resolve the value of the parameter
    fn resolve(self, state: &[i64], relative_base: i64) -> i64 {
        match self.mode {
            // direct: simply return the value at this address
            ParameterMode::Immediate => state[self.arg_pos as usize],
            // indirect: return the value at the address stored at this address
            ParameterMode::Position => state[state[self.arg_pos as usize] as usize],
            // relative: return the value at the address stored at this address + relative base
            ParameterMode::Relative => state[(relative_base + state[(self.arg_pos) as usize]) as usize]
        }
    }

    // resolve the position of the value of the parameter
    fn resolve_pos(self, state: &[i64], relative_base: i64) -> i64 {
        match self.mode {
            // direct: simply return this address
            ParameterMode::Immediate => self.arg_pos,
            // indirect: return the value at this address (which is the address of the value)
            ParameterMode::Position => state[self.arg_pos as usize],
            // relative: return the value at this address + relative base (which is the address of the value)
            ParameterMode::Relative => relative_base + state[( self.arg_pos) as usize]
        }
    }
}

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    Jnz(Parameter, Parameter),
    JZ(Parameter, Parameter),
    LT(Parameter, Parameter, Parameter),
    EQ(Parameter, Parameter, Parameter),
    AdjustRelativeBase(Parameter),
    Quit
}

struct ParameterModes(i64);

impl ParameterModes {
    fn pop(&mut self) -> ParameterMode {
        let result = ParameterMode::from(self.0 % 10);
        self.0 /= 10;
        result
    }
}

impl From<(usize, i64)> for Instruction {
    fn from(data: (usize, i64)) -> Self {
        let position = data.0;
        let position = position as i64;
        let instruction_value = data.1;
        let opcode = instruction_value % 100;
        let mut modes = ParameterModes(instruction_value / 100);


        match opcode {
            1 => Instruction::Add(
                Parameter::new(modes.pop(), position + 1),
                Parameter::new(modes.pop(), position + 2),
                Parameter::new(modes.pop(), position + 3)
            ),
            2 => Instruction::Multiply(
                Parameter::new(modes.pop(), position + 1),
                Parameter::new(modes.pop(), position + 2),
                Parameter::new(modes.pop(), position + 3)
            ),
            3 => Instruction::Input(
                Parameter::new(modes.pop(), position + 1),
            ),
            4 => Instruction::Output(
                Parameter::new(modes.pop(), position + 1),
            ),
            5 => Instruction::Jnz(
                Parameter::new(modes.pop(), position + 1),
                Parameter::new(modes.pop(), position + 2),
            ),
            6 => Instruction::JZ(
                Parameter::new(modes.pop(), position + 1),
                Parameter::new(modes.pop(), position + 2),
            ),
            7 => Instruction::LT(
                Parameter::new(modes.pop(), position + 1),
                Parameter::new(modes.pop(), position + 2),
                Parameter::new(modes.pop(), position + 3)
            ),
            8 => Instruction::EQ(
                Parameter::new(modes.pop(), position + 1),
                Parameter::new(modes.pop(), position + 2),
                Parameter::new(modes.pop(), position + 3)
            ),
            9 => Instruction::AdjustRelativeBase(
                Parameter::new(modes.pop(), position + 1),
            ),
            99 => Instruction::Quit,
            opcode => panic!("unknown opcode: {}", opcode)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DisplayMode {
    Primitive,
    Proper
}

#[derive(Debug, Clone)]
pub struct StateMachine {
    state: Vec<i64>,
    position: usize,
    done: bool,
    display: DisplayMode,
    relative_base: i64,
}

impl From<&str> for StateMachine {
    fn from(s: &str) -> Self {
        let mut machine = StateMachine {
            state: parse_ints(s, ","),
            position: 0,
            done: false,
            display: DisplayMode::Proper,
            relative_base: 0,
        };
        machine.state.resize(10000, 0);
        machine
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // a first iteration of the state machine did not implement output
        // and instead had you read the first value of the tape
        // for backwards compatibility reasons we implement display for the state machine outputting
        // the first value of the tape
        // in future iterations display the output vector instead.
        match self.display {
            DisplayMode::Primitive => Display::fmt(&self.state[0], f),
            DisplayMode::Proper => unimplemented!()
            // DisplayMode::Proper => Display::fmt(&self.output.last().unwrap(), f),
        }
    }
}

impl StateMachine
{

    pub fn run(&mut self, io: &mut dyn IO) {
        if self.done {
            return
        }
        loop {
            self.next(io);
            if self.done {
                break
            }
        }
    }

    pub(crate) fn run_until_output_or_complete(&mut self, io: &mut dyn IO) {
        if self.done {
            return
        }
        loop {
            // let output_len = self.output.len();
            self.next(io);
            if self.done || io.has_output() {
                break
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn get_state(&self, idx: usize) -> i64 {
        self.state[idx]
    }

    pub fn set_state(&mut self, idx: usize, value: i64) {
        self.state[idx] = value;
    }

    pub fn next(&mut self, io: &mut dyn IO) {
        if self.done {
            return
        }
        let command = Instruction::from((self.position, self.state[self.position]));
        match command {
            Instruction::Add(a, b, y) => {
                let y = y.resolve_pos(&self.state, self.relative_base) as usize;
                self.state[y] = a.resolve(&self.state, self.relative_base) + b.resolve(&self.state, self.relative_base);
                self.position += 4;
            }
            Instruction::Multiply(a, b, y) => {
                let y = y.resolve_pos(&self.state, self.relative_base) as usize;
                self.state[y] = a.resolve(&self.state, self.relative_base) * b.resolve(&self.state, self.relative_base);
                self.position += 4;
            }
            Instruction::Input(to) => {
                // println!("trying to read input");
                let y = to.resolve_pos(&self.state, self.relative_base) as usize;
                self.state[y] = io.read();
                self.position += 2;
            }
            Instruction::Output(from) => {
                io.write(from.resolve(&self.state, self.relative_base));
                self.position += 2;
            }
            Instruction::Quit => {
                self.done = true;
                self.position += 1;
            }
            Instruction::Jnz(t, p) => {
                if t.resolve(&self.state, self.relative_base) != 0 {
                    self.position = p.resolve(&self.state, self.relative_base) as usize;
                } else {
                    self.position += 3;
                }
            }
            Instruction::JZ(t, p) => {
                if t.resolve(&self.state, self.relative_base) == 0 {
                    self.position = p.resolve(&self.state, self.relative_base) as usize;
                } else {
                    self.position += 3;
                }
            }
            Instruction::LT(a, b, y) => {
                let y = y.resolve_pos(&self.state, self.relative_base) as usize;
                if a.resolve(&self.state, self.relative_base) < b.resolve(&self.state, self.relative_base) {
                    self.state[y] = 1;
                } else {
                    self.state[y] = 0;
                }
                self.position += 4;
            }
            Instruction::EQ(a, b, y) => {
                let y = y.resolve_pos(&self.state, self.relative_base) as usize;
                if a.resolve(&self.state, self.relative_base) == b.resolve(&self.state, self.relative_base) {
                    self.state[y] = 1;
                } else {
                    self.state[y] = 0;
                }
                self.position += 4;
            }
            Instruction::AdjustRelativeBase(a) => {
                self.relative_base += a.resolve(&self.state, self.relative_base);
                self.position += 2;
            }
        }
    }
}

pub trait IO {
    fn read(&mut self) -> i64;
    fn write(&mut self, value: i64);
    fn has_output(&self) -> bool {
        unimplemented!()
    }
    fn push_input(&mut self) -> i64 {
        unimplemented!()
    }

    fn pop_output(&mut self) -> i64 {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueueIO {
    pub input: VecDeque<i64>,
    pub output: Vec<i64>,
}

impl QueueIO {
    pub fn new() -> Self {
        Default::default()
    }
}

impl IO for QueueIO {
    fn read(&mut self) -> i64 {
        self.input.pop_front().expect("looked for input where there was none")
    }

    fn write(&mut self, value: i64) {
        self.output.push(value);
    }

    fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    fn push_input(&mut self) -> i64 {
        self.input.push_back(0);
        0
    }

    fn pop_output(&mut self) -> i64 {
        self.output.pop().expect("looked for output where there was none")
    }
}

pub enum IOOperation<'v> {
    Read(&'v mut Option<i64>),
    Write(i64)
}

pub struct AsyncIO<'a> {
    handler: Box<dyn FnMut(IOOperation) -> anyhow::Result<()> + 'a>,
}

impl <'a> AsyncIO<'a> {
    pub fn new<F>(handler: F) -> Self
        where F: FnMut(IOOperation) -> anyhow::Result<()> + 'a
    {
        Self {
            handler: Box::new(handler)
        }
    }
}

impl <'a> IO for AsyncIO<'a> {
    fn read(&mut self) -> i64 {
        let mut result = None;
        (self.handler)(IOOperation::Read(&mut result)).expect("failed to read");
        result.expect("failed to read")
    }

    fn write(&mut self, value: i64) {
        (self.handler)(IOOperation::Write(value)).expect("failed to write");
    }
}

