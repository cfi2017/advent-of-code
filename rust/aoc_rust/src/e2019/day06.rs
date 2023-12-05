use std::collections::HashMap;
use std::iter;

use crate::aoc::Puzzle;

pub struct Day06;

#[derive(Debug, Clone)]
pub struct Orbits {
    orbits: HashMap<String, Vec<String>>,
    cache: HashMap<String, i32>,
}

impl Orbits {
    pub fn get_steps(&mut self, key: &str) -> i32 {
        match self.cache.get(key) {
            Some(cached) => *cached,
            None => {
                let result = match self.orbits.get(key) {
                    Some(vec) => vec.to_owned().iter().map(|x| self.get_steps(x) + 1).sum(),
                    None => 0,
                };
                self.cache.insert(key.to_owned(), result);
                result
            }
        }
    }
}

impl Puzzle<Orbits, i32, i32, 2019, 6> for Day06 {
    fn sanitize_input(&self, input: &str) -> Orbits {
        let mut orbits = HashMap::new();
        for line in input.lines() {
            let mut parts = line.split(')');
            let a = parts.next().unwrap().to_string();
            let b = parts.next().unwrap().to_string();
            orbits.entry(a).or_insert(Vec::new()).push(b);
        }
        Orbits {
            orbits,
            cache: HashMap::new(),
        }
    }

    fn solve_a(&self, mut input: Orbits) -> i32 {
        input.orbits.to_owned().keys().map(|x| input.get_steps(x)).sum()
    }

    fn solve_b(&self, input: Orbits) -> i32 {
        let reverse: HashMap<&String, &String> = HashMap::from_iter(input.orbits.iter().flat_map(|(k, v)| v.iter().map(move |x| (x, k))));
        let succ = |x: &String| reverse.get(x).map(|x| x.to_owned()).cloned();
        let san = iter::successors(Some("SAN".to_string()), succ).collect::<Vec<_>>();
        let you = iter::successors(Some("YOU".to_string()), succ).collect::<Vec<_>>();
        let common = san.iter().rev().zip(you.iter().rev()).take_while(|(a, b)| a == b).count();
        (san.len() + you.len() - 2 * (common + 1)) as i32
    }
}
