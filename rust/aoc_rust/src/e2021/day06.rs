use std::str::FromStr;
use crate::aoc::{Puzzle, read_input};

pub struct Day06;

#[derive(Copy, Clone)]
struct Swarm {
    fish: [i64; 9],
    simulation_length: i64,
    simulated_for: i64,
}

impl FromStr for Swarm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish = s.split(",")
            .filter(|s| !s.is_empty())
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .fold([0; 9], |mut acc, v| {
                acc[v as usize] += 1;
                acc
            });
        Ok(Swarm{
            fish,
            simulation_length: 0,
            simulated_for: 0,
        })
    }
}

impl Swarm {
    fn tick(&mut self) {
        self.fish.rotate_left(1);
        self.fish[6] += self.fish[8];
    }

    fn simulate(&mut self) {
        for _ in self.simulated_for..self.simulation_length {
            self.tick();
            self.simulated_for += 1;
        }
    }

    fn sum(&self) -> i64 {
        self.fish.iter().fold(0, |a, b| a + b)
    }
}

impl Puzzle<Swarm, i64, i64, 2021, 6> for Day06 {

    fn sanitize_input(&self, input: &str) -> Swarm {
        Swarm::from_str(&input).unwrap()
    }

    fn solve_a(&self, mut input: Swarm) -> i64 {
        input.simulation_length = 80;
        input.simulate();
        input.sum()
    }

    fn solve_b(&self, mut input: Swarm) -> i64 {
        input.simulation_length = 256;
        input.simulate();
        input.sum()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "3,4,3,1,2";
        let mut input = Day06.sanitize_input(input);
        assert_eq!(Day06.solve_a(input.clone()), 5934);
        assert_eq!(Day06.solve_b(input.clone()), 26984457539);
    }
}
