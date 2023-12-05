use std::ops::{Add, Sub};
use std::str::FromStr;
use itertools::Itertools;
use crate::aoc::Puzzle;
use crate::aoc_boilerplate;

pub struct PuzzleDay;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    const fn zero() -> Self {
        Point2D { x: 0, y: 0 }
    }

    const fn one() -> Self {
        Point2D { x: 1, y: 1 }
    }

    const fn up() -> Self {
        Point2D { x: 0, y: 1 }
    }

    const fn down() -> Self {
        Point2D { x: 0, y: -1 }
    }

    const fn left() -> Self {
        Point2D { x: -1, y: 0 }
    }

    const fn right() -> Self {
        Point2D { x: 1, y: 0 }
    }

    pub fn new(x: i64, y: i64) -> Self {
        Point2D { x, y }
    }

    pub fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.is_positive() && self.x < width as i64 && self.y < height as i64
    }

}

impl Add for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Point2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T: Clone> {
    data: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    fn get(&self, point: Point2D) -> Option<T> {
        self.data.get(point.y as usize).map(|row| row.get(point.x as usize)).flatten().cloned()
    }
}

impl FromStr for Grid<u8> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.bytes().collect())
            .collect();
        Ok(Grid {
            data
        })
    }
}

fn get_nums(grid: &Grid<u8>) -> Vec<(i32, Point2D, usize)> {
    let mut nums: Vec<(i32, Point2D, usize)> = Vec::new();
    for (y, line) in grid.data.iter().enumerate() {
        let mut buf = 0;
        let mut len = 0;
        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                buf = buf * 10 + (*c as i32 - b'0' as i32);
                len += 1;
            } else if buf > 0 {
                nums.push((buf, Point2D { x: x as i64 - len, y: y as i64 }, len as usize));
                buf = 0;
                len = 0;
            }
        }
        if buf > 0 {
            nums.push((buf, Point2D { x: line.len() as i64 - len, y: y as i64 }, len as usize));
        }
    }
    nums
}

pub fn gen_border_positions(p: Point2D, len: usize, width: usize, height: usize) -> Vec<Point2D> {
    let mut positions = Vec::new();
    // push left
    positions.push(p + Point2D::left());
    positions.push(p + Point2D::left() + Point2D::up());
    positions.push(p + Point2D::left() + Point2D::down());
    for i in 0..len {
        positions.push(p + Point2D::up() + Point2D::new(i as i64, 0));
        positions.push(p + Point2D::down() + Point2D::new(i as i64, 0));
    }
    // push right
    positions.push(p + Point2D::new(len as i64, 0));
    positions.push(p + Point2D::up() + Point2D::new(len as i64, 0));
    positions.push(p + Point2D::down() + Point2D::new(len as i64, 0));
    positions.iter().filter(|p| p.in_bounds(width, height)).cloned().collect()
}

aoc_boilerplate!(2023, 3, sanitize_input, solve_a, solve_b);

    pub fn sanitize_input(input: &str) -> Grid<u8> {
        input.parse().unwrap()
    }

    pub fn solve_a(input: Grid<u8>) -> i32 {
        let mut nums = get_nums(&input);
        let width = input.data[0].len();
        let height = input.data.len();

        nums.iter()
            .map(|(num, point, len)| (num, gen_border_positions(*point, *len, width, height)))
            .filter(|(_, positions)| positions.iter().any(|p| input.get(*p).is_some_and(|c| !c.is_ascii_digit() && c != b'.')))
            .map(|(num, _)| *num)
            .sum()
    }

    pub fn solve_b(input: Grid<u8>) -> i32 {
        let mut nums = get_nums(&input);
        let width = input.data[0].len();
        let height = input.data.len();

        let map = nums.iter()
            .map(|(num, point, len)| gen_border_positions(*point, *len, width, height)
                .into_iter()
                .filter(|p| {
                    input.get(*p).is_some_and(|c| c == b'*')
                })
                .map(|p| (p, (*num, *point)))
            ).flatten()
            .into_group_map();

        map.iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0].0 * v[1].0)
            .sum()
    }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::add_test;

    add_test!(test_solve_a_example, solve_a, r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#, 4361);

    add_test!(test_solve_b_example, solve_b, r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#, 467835);


}
