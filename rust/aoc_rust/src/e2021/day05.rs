use std::collections::HashMap;
use std::str::FromStr;
use crate::aoc::Puzzle;

pub struct Day05;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i32> = s.split(',')
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();
        if parts.len() != 2 {
            return Err(());
        }
        Ok(Point {
            x: parts[0],
            y: parts[1],
        })
    }
}

#[derive(Copy, Clone)]
struct Line {
    from: Point,
    to: Point,

}

impl Line {
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.from.x == self.to.x {
            let from_y = self.from.y.min(self.to.y);
            let to_y = self.from.y.max(self.to.y);
            for y in from_y..to_y +1 {
                points.push(Point {x: self.from.x, y})
            }
        } else if self.from.y == self.to.y {
            let from_x = self.from.x.min(self.to.x);
            let to_x = self.from.x.max(self.to.x);
            for x in from_x..to_x +1 {
                points.push(Point {x, y: self.from.y})
            }
        } else {
            // diagonal
            if self.from.x < self.to.x {
                if self.from.y < self.to.y {
                    // diagonal line towards bottom right
                    for i in 0..self.to.x-self.from.x {
                        points.push(Point {x: self.from.x+i, y: self.from.y+i});
                    }
                } else {
                    // diagonal line towards top right
                    for i in 0..self.to.x-self.from.x {
                        points.push(Point {x: self.from.x+i, y: self.from.y-i});
                    }
                }
            } else if self.from.y < self.to.y {
                // diagonal line towards bottom left
                for i in 0..self.from.x-self.to.x {
                    points.push(Point {x: self.from.x-i, y: self.from.y+i});
                }
            } else {
                // diagonal line towards top left
                for i in 0..self.from.x-self.to.x {
                    points.push(Point {x: self.from.x-i, y: self.from.y-i});
                }
            }
        }
        points
    }

    fn is_straight(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Point> = s.split(" -> ")
            .map(str::parse::<Point>)
            .map(Result::unwrap)
            .collect();
        if parts.len() != 2 {
            return Err(());
        }
        Ok(Line {
            from: parts[0],
            to: parts[1],
        })
    }
}

impl Puzzle<Vec<Line>, i32, i32, 2021, 5> for Day05 {

    fn sanitize_input(&self, input: &str) -> Vec<Line> {
        input.split('\n')
            .map(str::trim)
            .filter(|str| !str.is_empty())
            .map(str::parse::<Line>)
            .map(Result::unwrap)
            .collect()
    }

    fn solve_a(&self, input: Vec<Line>) -> i32 {
        let mut points: HashMap<Point, i32> = HashMap::new();
        for point in input.iter()
            .filter(|line| line.is_straight())
            .flat_map(Line::points) {
            *points.entry(point).or_insert(0) += 1;
        }

        points.values().filter(|v| **v > 1).count() as i32
    }

    fn solve_b(&self, input: Vec<Line>) -> i32 {
        let mut points: HashMap<Point, i32> = HashMap::new();
        for point in input.iter()
            .flat_map(Line::points) {
            *points.entry(point).or_insert(0) += 1;
        }

        points.values().filter(|v| **v > 1).count() as i32
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {

        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input = Day05.sanitize_input(input);
        assert_eq!(Day05.solve_a(input.clone()), 5);
        assert_eq!(Day05.solve_b(input.clone()), 12);
    }

    #[test]
    fn test_lines() {
        let line = "1,1 -> 1,3".parse::<Line>().unwrap();
        println!("{:?}", line.points());
        let line = "9,7 -> 7,7".parse::<Line>().unwrap();
        println!("{:?}", line.points());
    }
}
