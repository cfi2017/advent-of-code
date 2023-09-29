use std::collections::HashSet;
use crate::aoc::Puzzle;
use crate::e2019::map2d::Map2D;
use crate::e2019::vec2::Vec2;

pub struct Day10;

impl Puzzle<Map2D, i32, i32, 2019, 10> for Day10 {
    fn sanitize_input(&self, input: &str) -> Map2D {
        let vec: Vec2<char> = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        Map2D::new(vec, '#', '.')
    }

    fn solve_a(&self, input: Map2D) -> i32 {
        let mut max = 0;
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
                }
            }
        }
        max as i32
    }

    fn solve_b(&self, input: Map2D) -> i32 {
        0
    }
}

