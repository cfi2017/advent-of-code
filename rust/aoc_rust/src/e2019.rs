use crate::aoc::Puzzle;
use crate::e2019::day03::Day03;
use crate::e2019::day04::Day04;
use crate::e2019::day05::Day05;
use crate::e2019::day06::Day06;
use crate::e2019::day07::Day07;
use crate::e2019::day08::Day08;
use crate::e2019::day09::Day09;
use crate::e2019::day10::Day10;
use crate::e2019::day11::Day11;
use crate::e2019::day12::Day12;
use crate::e2019::day13::Day13;
use crate::e2019::day14::Day14;

pub mod state_machine;
pub mod vec2;
pub mod vec3;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub mod day12;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day13;
pub mod day14;
mod map2d;

pub fn run() {
    // todo: day 1 as puzzle format
    day01::PuzzleDay.run();
    day02::PuzzleDay.run();
    // todo: fix day 3
    // day 3 is also very slow
    Day03.run();
    Day04.run();
    // todo: fix day 5 (divergence between part a and b breaks part a)
    Day05.run();
    Day06.run();
    Day07.run();
    Day08.run();
    Day09.run();
    Day10.run();
    Day11.run();
    Day12.run();
    Day13.run();
    Day14.run();

}
