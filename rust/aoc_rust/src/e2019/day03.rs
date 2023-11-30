use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Index};
use std::str::FromStr;
use crate::aoc::Puzzle;

pub struct Day03;

#[derive(Copy, Clone, Debug, Eq, Hash)]
struct WeightedPoint {
    point: Point,
    weight: i32,
}

impl Add for WeightedPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        WeightedPoint {
            point: self.point + rhs.point,
            weight: self.weight + rhs.weight,
        }
    }
}

impl WeightedPoint {
    pub const ZERO: WeightedPoint = WeightedPoint { point: Point::ZERO, weight: 0 };

    fn new(x: i32, y: i32, weight: i32) -> WeightedPoint {
        WeightedPoint {
            point: Point {
                x,
                y,
            },
            weight,
        }
    }

    fn to_point(&self) -> Point {
        self.point
    }
}

impl PartialEq for WeightedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0, };

    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x, y
        }
    }

}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn get_points(&self, start: WeightedPoint) -> (Vec<WeightedPoint>, WeightedPoint) {
        let mut current = start;
        let mut points = Vec::new();
        let mut mutator: WeightedPoint = WeightedPoint::ZERO;
        let mut length = 0;

        match self {
            Instruction::Up(x) => {
                length = *x;
                mutator = WeightedPoint::new(0, 1, 1)
            },
            Instruction::Down(x) => {
                length = *x;
                mutator = WeightedPoint::new(0, -1, 1)
            }
            Instruction::Left(x) => {
                length = *x;
                mutator = WeightedPoint::new(-1, 0, 1)
            },
            Instruction::Right(x) => {
                length = *x;
                mutator = WeightedPoint::new(1, 0, 1)
            }

        };
        for _ in 0..length {
            current = current + mutator;
            points.push(current);
        }
        return (points, current);
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d: char = s.as_bytes()[0] as char;
        let num = &s[1..];
        let num: i32 = num.parse().unwrap();
        Ok(match d {
            'U' => Instruction::Up(num),
            'D' => Instruction::Down(num),
            'L' => Instruction::Left(num),
            'R' => Instruction::Right(num),
            _ => panic!("invalid instruction")
        })
    }
}

#[derive(Clone)]
pub struct Wire {
    instructions: Vec<Instruction>,
}

impl Wire {
    fn get_points(&self) -> Vec<WeightedPoint> {
        let mut points = Vec::new();
        let mut position = WeightedPoint::ZERO;
        for instruction in &self.instructions {
            let (new_points, new_position) = instruction.get_points(position);
            for point in new_points {
                points.push(point);
            }
            position = new_position;
        }
        return points;
    }
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .split(",")
            .map(str::parse::<Instruction>)
            .map(Result::unwrap)
            .collect();
        Ok(Wire {
            instructions,
        })
    }
}



impl Puzzle<(Wire, Wire), i32, i32, 2019, 3> for Day03 {

    fn sanitize_input(&self, input: &str) -> (Wire, Wire) {
        let wires: Vec<Wire> = input.split("\n")
            .filter(|x| !x.is_empty())
            .map(str::parse::<Wire>)
            .map(Result::unwrap)
            .collect();
        return (wires[0].clone(), wires[1].clone())
    }

    fn solve_a(&self, input: (Wire, Wire)) -> i32 {
        let points_a: HashSet<Point, RandomState> = HashSet::from_iter(input.0.get_points().iter().map(WeightedPoint::to_point));
        let points_b: HashSet<Point, RandomState> = HashSet::from_iter(input.1.get_points().iter().map(WeightedPoint::to_point));
        let intersecting: HashSet<_> = points_a.intersection(&points_b).collect();
        let mut arr: Vec<_> = intersecting.iter().collect();
        arr.sort_by(|p1, p2| distance_to_root(p1).cmp(&distance_to_root(p2)));
        let result = arr.get(0).unwrap();
        return distance_to_root(result);
    }

    fn solve_b(&self, input: (Wire, Wire)) -> i32 {
        let points_a: HashSet<Point, RandomState> = HashSet::from_iter(input.0.get_points().iter().map(WeightedPoint::to_point));
        let points_b: HashSet<Point, RandomState> = HashSet::from_iter(input.1.get_points().iter().map(WeightedPoint::to_point));
        let intersecting: HashSet<_> = points_a.intersection(&points_b).collect();
        let points_a = input.0.get_points();
        let points_a: HashSet<&WeightedPoint, RandomState> = HashSet::from_iter(points_a.iter());
        let points_b = input.1.get_points();
        let points_b: HashSet<&WeightedPoint, RandomState> = HashSet::from_iter(points_b.iter());
        let b = intersecting.iter().map(|pi| {
            // map each intersection point to the minimum sum available for all available points
            // then find the minimum of those
            let points_a = points_a.iter().filter(|p| p.point == **pi).map(|p| p.weight).min();
            let points_b = points_b.iter().filter(|p| p.point == **pi).map(|p| p.weight).min();
            points_a.unwrap() + points_b.unwrap()
        }).min().unwrap();
        return b;
    }
}

fn distance_to_root(p: &Point) -> i32 {
    return distance(p, &Point::ZERO)
}

fn distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "R8,U5,L5,D3\
U7,R6,D4,L4";

        let input = Day03.sanitize_input(input);
        assert_eq!(Day03.solve_a(input.clone()), 6);
        assert_eq!(Day03.solve_a(input.clone()), 6);

        let input = Day03.sanitize_input(&Day03.get_input());
        println!("a: {}", Day03.solve_a(input));
    }
}
