use std::collections::HashSet;
use crate::aoc::Puzzle;
use crate::e2019::map2d::{Map2D, Position};
use crate::e2019::vec2::Vec2;

pub struct Day10;

fn farey(n: i64) -> Vec<Position> {
    let mut a = Position::new(0, 1);
    let mut b = Position::new(1, n);
    let mut result = vec![a];
    while b.x <= n {
        let k = (n + a.y) / b.y;
        let c = b;
        b = Position::new(k * b.x - a.x, k * b.y - a.y);
        a = c;
        result.push(a);
    }
    result
}

fn find_station_position(input: &Map2D) -> (Position, i32) {
    let mut max = 0;
    let mut best = Position::new(0, 0);
    for pos in input.iter_points() {
        if input.get(pos) == input.obstacle {
            let mut vectors = HashSet::new();
            for other in input.iter_points() {
                if input.get(other) == input.obstacle {
                    if let Some(v) = input.can_see(pos, other) {
                        vectors.insert(v);
                    }
                }
            }
            if vectors.len() > max {
                max = vectors.len();
                best = pos;
            }
        }
    }
    (best, max as i32)
}


impl Puzzle<Map2D, i32, i32, 2019, 10> for Day10 {
    fn sanitize_input(&self, input: &str) -> Map2D {
        let vec: Vec2<char> = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        Map2D::new(vec, '#', '.')
    }

    fn solve_a(&self, input: Map2D) -> i32 {
        let (_, max) = find_station_position(&input);
        max
    }

    fn solve_b(&self, input: Map2D) -> i32 {
        let (station, _) = find_station_position(&input);
        let mut wtf: Vec<_> = farey(input.width().max(input.height()) as i64);
        assert_eq!(wtf.first(), Some(&Position::new(0, 1)));
        assert_eq!(wtf.last(), Some(&Position::new(1, 1)));
        wtf.extend(wtf.clone().into_iter().map(Position::invert).rev().skip(1));
        assert_eq!(wtf.last(), Some(&Position::new(1, 0)));
        wtf.extend(wtf.clone().into_iter().map(|p| Position::new(p.x, -p.y)).rev().skip(1));
        assert_eq!(wtf.last(), Some(&Position::new(0, -1)));
        wtf.extend(wtf.clone().into_iter().map(|p| Position::new(-p.x, p.y)).rev().skip(1));
        assert_eq!(wtf.last(), Some(&Position::new(0, 1)));
        wtf.pop();
        wtf = wtf.into_iter().map(|p| Position::new(-p.y, p.x)).collect();

        let mut todo: Vec<i64> = vec![];

        for pos in &wtf {
            let mut current = station + *pos;

            while 0 <= current.y && current.y < input.height() as i64 && 0 <= current.x && current.x < input.width() as i64 {
                if input.get(current) == input.obstacle {
                    let out = current.x * 100 + current.y;
                    todo.push(out);
                }
                current += *pos;
            }
        }

        *todo.iter().take(200).last().unwrap() as i32
    }
}

