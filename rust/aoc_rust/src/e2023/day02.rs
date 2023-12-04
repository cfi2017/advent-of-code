use std::collections::HashMap;
use std::str::FromStr;
use crate::aoc::Puzzle;

pub struct PuzzleDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(())
        }
    }
}

impl Color {
    fn is_possible(&self, n: i32) -> bool {
        match self {
            Color::Red => n <= 12,
            Color::Green => n <= 13,
            Color::Blue => n <= 14,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    id: i32,
    rounds: Vec<HashMap<Color, i32>>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let id = parts.next().unwrap().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        let rounds = parts.next().unwrap().split("; ")
            .map(|x| x.split(", ")
                .map(|x| x.split(' '))
                .map(|mut x| (x.next().unwrap().parse::<i32>().unwrap(), x.next().unwrap().parse::<Color>().unwrap()))
                .map(|(n, color)| (color, n))
                .collect::<HashMap<Color, i32>>())
            .collect();
        Ok(Game {
            id, rounds
        })
    }
}

impl Game {
    fn possible(&self) -> bool {
        self.rounds.iter().all(|round| round.iter().all(|(color, n)| color.is_possible(*n)))
    }

    fn power(&self) -> i32 {
        let mut max_cubes = (0, 0, 0);
        for round in &self.rounds {
            for (color, n) in round {
                match color {
                    Color::Red => {
                        if max_cubes.0 < *n {
                            max_cubes.0 = *n;
                        }
                    },
                    Color::Green => {
                        if max_cubes.1 < *n {
                            max_cubes.1 = *n;
                        }
                    },
                    Color::Blue => {
                        if max_cubes.2 < *n {
                            max_cubes.2 = *n;
                        }
                    },
                };
            }
        }
        max_cubes.0 * max_cubes.1 * max_cubes.2
    }
}

impl Puzzle<Vec<Game>, i32, i32, 2023, 2> for PuzzleDay {
    fn sanitize_input(&self, input: &str) -> Vec<Game> {
        input.split("\n")
            .filter(|x| !x.is_empty())
            .map(str::parse::<Game>)
            .map(Result::unwrap).collect()
    }

    fn solve_a(&self, games: Vec<Game>) -> i32 {
        games.iter().filter(|x| x.possible()).map(|x| x.id).sum()
    }

    fn solve_b(&self, games: Vec<Game>) -> i32 {
        games.iter().map(|x| x.power()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::add_test;

    #[test]
    fn test_sanitize_input() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let games = PuzzleDay.sanitize_input(input);
    }

    add_test!(test_solve_a_example, solve_a, r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#, 8);

    add_test!(test_solve_b_example, solve_b, r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#, 2286);

}
