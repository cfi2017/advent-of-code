use std::ops::{Add, AddAssign, Div, Sub};
use crate::e2019::vec2::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }

    pub fn invert(self) -> Position {
        Position::new(self.y, self.x)
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i64> for Position {
    type Output = Position;

    fn div(self, rhs: i64) -> Self::Output {
        Position {
            x: (self.x / rhs),
            y: (self.y / rhs),
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = *self + rhs;
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Map2D {
    // map of chars
    map: Vec2<char>,
    // these chars are considered obstacles
    pub(crate) obstacle: char,
    empty: char,
}

// operations on a 2d map
impl Map2D {
    pub fn new(map: Vec2<char>, obstacle: char, empty: char) -> Self {
        Map2D { map, obstacle, empty }
    }

    pub fn can_see(&self, a: Position, b: Position) -> Option<Position> {
        if a == b {
            return None;
        }
        let diff = b - a;
        let minimum_vec = diff / gcd(diff.x, diff.y);
        Some(minimum_vec)
    }

    pub fn iter_points(&self) -> impl Iterator<Item=Position> {
        let width = self.map[0].len();
        let height = self.map.len();
        (0..width).flat_map(move |x| (0..height).map(move |y| Position::new(x as i64, y as i64)))
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn get(&self, pos: Position) -> char {
        self.map[pos.y as usize][pos.x as usize]
    }
}

// common stuff

pub fn gcd(mut m: i64, mut n: i64) -> i64 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}
pub fn farey(n: i64) -> Vec<Position> {
    let mut v: Vec<Position> = Vec::new();
    let mut ab = (0, 1);
    let mut cd = (1, n);
    v.push(ab.into());

    while cd.0 < n {
        let k = (n + ab.1) / cd.1;
        let ef = (k * cd.0 - ab.0, k * cd.1 - ab.1);
        ab = cd;
        cd = ef;
        v.push(ab.into());
    }
    v
}
