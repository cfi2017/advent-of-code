use std::collections::HashMap;
use crate::aoc::Puzzle;
use crate::e2019::map2d::Position;
use crate::e2019::state_machine::{QueueIO, StateMachine};

pub struct Day11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colour {
    Black,
    White,
}

impl From<i64> for Colour {
    fn from(value: i64) -> Self {
        match value {
            0 => Colour::Black,
            1 => Colour::White,
            _ => panic!("invalid colour"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<i64> for Direction {
    fn from(value: i64) -> Self {
        match value {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("invalid turn direction"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Turtle {
    machine: StateMachine,
    position: Position,
    direction: Direction,
}

impl Turtle {
    pub fn new(machine: StateMachine) -> Self {
        Turtle {
            machine,
            position: Position::new(0, 0),
            direction: Direction::Up,
        }
    }

    pub fn run(mut self, mut canvas: HashMap<Position, Colour>) -> HashMap<Position, Colour> {
        // first position is black
        let mut io = QueueIO::new();
        io.input.push_back(*canvas.get(&self.position).unwrap_or(&Colour::Black) as i64);

        loop {
            self.machine.run_until_output_or_complete(&mut io);
            if self.machine.is_done() {
                break
            }
            let colour: Colour = io.output.pop().unwrap().into();
            canvas.insert(self.position, colour);
            self.machine.run_until_output_or_complete(&mut io);
            let turn: Direction = io.output.pop().unwrap().into();

            self.direction = match turn {
                Direction::Left => match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                },
                Direction::Right => match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
                _ => unreachable!()
            };

            self.position += match self.direction {
                Direction::Up => Position::new(0, 1),
                Direction::Down => Position::new(0, -1),
                Direction::Left => Position::new(-1, 0),
                Direction::Right => Position::new(1, 0),
            };
            io.input.push_back(*canvas.get(&self.position).unwrap_or(&Colour::Black) as i64);

        }

        canvas
    }
}

impl Puzzle<StateMachine, i32, i32, 2019, 11> for Day11 {
    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine::from(input)
    }

    fn solve_a(&self, machine: StateMachine) -> i32 {

        let turtle = Turtle::new(machine);
        let canvas: HashMap<Position, Colour> = HashMap::new();
        let canvas = turtle.run(canvas);
        canvas.len() as i32
    }

    fn solve_b(&self, machine: StateMachine) -> i32 {
        let turtle = Turtle::new(machine);
        let mut canvas: HashMap<Position, Colour> = HashMap::new();
        canvas.insert(Position::new(0, 0), Colour::White);

        let canvas = turtle.run(canvas);
        // render canvas
        let max_width = canvas.iter().filter(|(_, c)| **c == Colour::White).map(|(p, _)| p.x).max().unwrap() + 1;
        let max_height = canvas.iter().filter(|(_, c)| **c == Colour::White).map(|(p, _)| p.y).max().unwrap() + 1;
        let min_width = canvas.iter().filter(|(_, c)| **c == Colour::White).map(|(p, _)| p.x).min().unwrap();
        let min_height = canvas.iter().filter(|(_, c)| **c == Colour::White).map(|(p, _)| p.y).min().unwrap();
        for y in (min_height..=max_height).rev() {
            for x in min_width..=max_width {
                let colour = canvas.get(&Position::new(x, y)).unwrap_or(&Colour::Black);
                print!("{}", match colour {
                    Colour::Black => ' ',
                    Colour::White => '#',
                });
            }
            println!();
        }

        canvas.len() as i32
    }
}
