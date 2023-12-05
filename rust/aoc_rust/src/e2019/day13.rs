
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::aoc::Puzzle;
use crate::e2019::map2d::Position;
use crate::e2019::state_machine::{AsyncIO, IOOperation, QueueIO, StateMachine};

pub struct Day13;

impl Puzzle<StateMachine, i32, i32, 2019, 13> for Day13 {
    fn sanitize_input(&self, input: &str) -> StateMachine {
        StateMachine::from(input)
    }

    fn solve_a(&self, mut input: StateMachine) -> i32 {
        let mut map: HashMap<Position, i64> = HashMap::new();

        let mut io = QueueIO::new();

        loop {
            input.run_until_output_or_complete(&mut io);
            if input.is_done() {
                break;
            }
            let x = io.output.pop().unwrap();
            input.run_until_output_or_complete(&mut io);
            let y = io.output.pop().unwrap();
            input.run_until_output_or_complete(&mut io);
            let tile = io.output.pop().unwrap();

            map.insert(Position::new(x, y), tile);
        }
        map.values().filter(|&&v| v == 2).count() as i32
    }

    fn solve_b(&self, mut input: StateMachine) -> i32 {
        // patch to play for free?
        input.set_state(0, 2);

        // game state
        let mut map: HashMap<Position, i64> = HashMap::new();
        let mut ball = Position::new(0, 0);
        let mut paddle = Position::new(0, 0);
        let mut pos = Position::new(0, 0);
        let mut nth_output = 0;
        let mut score = 0;
        
        // run in a separate scope to drop io before returning score
        {
            let mut io = AsyncIO::new(|op| {
                match op {
                    IOOperation::Read(value) => {
                        *value = Some(match paddle.x.cmp(&ball.x) {
                            Ordering::Less => 1,
                            Ordering::Equal => 0,
                            Ordering::Greater => -1,
                        })
                    }
                    IOOperation::Write(value) => {
                        match nth_output {
                            0 => pos.x = value,
                            1 => pos.y = value,
                            2 => {
                                if pos == Position::new(-1, 0) {
                                    score = value;
                                } else {
                                    map.insert(pos, value);
                                    match value {
                                        3 => paddle = pos,
                                        4 => ball = pos,
                                        _ => (),
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                        nth_output = (nth_output + 1) % 3;
                    }
                };
                Ok(())
            });

            input.run(&mut io);
        }
        score as i32
    }
}
