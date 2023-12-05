use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::aoc::Puzzle;
use crate::aoc_boilerplate;

pub struct PuzzleDay;

#[derive(Debug, Clone)]
pub struct Scratchcard {
    id: i32,
    winning: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("   ", " ");
        let s = s.replace("  ", " ");
        let mut parts = s.split(": ");
        let id = parts.next().unwrap().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        let mut parts = parts.next().unwrap().split(" | ");
        let winning = parts.next().unwrap().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        let numbers = parts.next().unwrap().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

        Ok(Scratchcard {
            id,
            winning,
            numbers,
        })
    }
}

aoc_boilerplate!(2023, 4, sanitize_input, solve_a, solve_b);

pub fn sanitize_input(input: &str) -> Vec<Scratchcard> {
    input.split('\n')
        .filter(|x| !x.is_empty())
        .map(str::parse::<Scratchcard>)
        .collect::<Result<Vec<Scratchcard>, ()>>().unwrap()
}

pub fn solve_a(input: Vec<Scratchcard>) -> i32 {
    let mut points = 0;
    for card in input {
        let l = card.winning.intersection(&card.numbers).count();
        if l > 0 {
            points += 1 << (l - 1);
        }
    }
    points
}

pub fn solve_b(input: Vec<Scratchcard>) -> i32 {
    let mut counts = HashMap::new();
    for card in input {
        *counts.entry(card.id).or_insert(0) += 1;
        let l = card.winning.intersection(&card.numbers).count();
        for i in 0..l {
            *counts.entry(card.id + 1 + i as i32).or_insert(0) += *counts.entry(card.id).or_default();
        }
    }
    counts.values().sum()
}


#[cfg(test)]
mod tests {
    use crate::add_test;
    use super::*;

    add_test!(test_solve_a_example, solve_a, r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#, 13);

    add_test!(test_solve_b_example, solve_b, r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#, 30);
}
