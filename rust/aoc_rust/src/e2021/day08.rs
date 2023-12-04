#![cfg(feature = "experimental")]
use std::collections::HashMap;
use std::str::FromStr;
use crate::aoc::Puzzle;

pub struct Day08;

/**
2 3 4 7 number of segments
1 7 4 8 number displayed


*/


struct Entry<'a> {
    mapping: HashMap<char, char>,
    patterns: HashMap<&'a str, &'a str>,
    symbols: HashMap<u8, u8>,
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'a> FromStr for Entry<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(" | ").collect();
        if parts.len() != 2 {
            return Err(());
        }
        let input = parts[0];
        let input: Vec<&str> = input.split(" ").collect();
        let output = parts[1];
        let output: Vec<&str> = output.split(" ").collect();
        Ok(Entry {
            input,
            output,
            mapping: HashMap::new(),
            patterns: HashMap::new(),
            symbols: HashMap::new(),
        })
    }
}

fn find_len_sorted(v: &Vec<&str>, i: usize) -> &str {
    let mut num = v.iter().find(|x| x.len() == i).unwrap().clone();
    num.sort();
    num
}

fn find_nonmatching(a: &str, b: &str) -> char {
    for char in a {
        if !b.contains(char) {
            return char
        }
    }
    for char in b {
        if !a.contains(char) {
            return char
        }
    }
    panic!("fix your code");
}

impl<'a> Entry<'a> {

    fn deduce(&mut self) {
        self.input.sort_by(|a, b| a.len().cmp(&b.len()));
        let one = find_len_sorted(&self.input, 2);
        self.patterns.insert("cf", one);
        let seven = find_len_sorted(&self.input, 3);
        self.patterns.insert("acf", seven);

        // deduce a
        let a = find_nonmatching(one, seven);
        self.mapping.insert('a', a);

        // find four, extract partial (bd)
        // find five + matching a + matching 1 + matching 1/2 from four partial => 3
        // extrapolate g from 3
        // currently unseen symbol is e
        // now known symbols are a, g, e
        // now known numbers are 1, 3, 4, 5, 7, 8 (missing 2, 6, 9)
        // find numbers with len(6) => has e? 6. no? 9.
        // final missing number is 2
    }

}

struct Digit {

}

impl Puzzle<Vec<String>, i32, i32, 2021, 8> for Day08 {

    fn sanitize_input(&self, _: &str) -> Vec<String> {
        unimplemented!()
    }

    fn solve_a(&self, _: Vec<String>) -> i32 {
        unimplemented!()
    }

    fn solve_b(&self, _: Vec<String>) -> i32 {
        unimplemented!()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        Day08.solve(vec![String::from("abc")]);
    }
}
