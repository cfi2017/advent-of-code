use itertools::Itertools;

use crate::aoc::Puzzle;
use crate::e2019::vec3::Vec3;

pub struct Day08;

type Base = i32;

impl Puzzle<Vec3<Base>, Base, String, 2019, 8> for Day08 {
    fn sanitize_input(&self, input: &str) -> Vec3<Base> {
        input.chars()
            .flat_map(|c| c.to_digit(10))
            .map(|c| c as Base)
            .chunks(25 * 6)
            .into_iter()
            .map(|chunk| {
                chunk
                    .chunks(25)
                    .into_iter()
                    .map(|row| {
                        row.collect()
                    }).collect()
            }).collect()
    }

    fn solve_a(&self, input: Vec3<Base>) -> Base {
        // outer vector are layers
        // inner vectors are rows
        // innermost vectors are pixels
        let mut min_zeroes = usize::MAX;
        let mut min_layer = 0;
        for (i, layer) in input.iter().enumerate() {
            let zeroes = layer.iter().flatten().filter(|&&p| p == 0).count();
            if zeroes < min_zeroes {
                min_zeroes = zeroes;
                min_layer = i;
            }
        }
        let layer = &input[min_layer];
        let ones = layer.iter().flatten().filter(|&&p| p == 1).count();
        let twos = layer.iter().flatten().filter(|&&p| p == 2).count();
        (ones * twos) as Base
    }

    fn solve_b(&self, input: Vec3<Base>) -> String {
        let mut result = input[0].clone();
        for layer in input[1..].iter() {
            for (row, result_row) in layer.iter().zip(result.iter_mut()) {
                for (pixel, result_pixel) in row.iter().zip(result_row.iter_mut()) {
                    if *result_pixel == 2 {
                        *result_pixel = *pixel;
                    }
                }
            }
        }
        for row in result.iter() {
            for pixel in row.iter() {
                if *pixel == 0 {
                    print!(" ");
                } else {
                    print!("{}", pixel);
                }
            }
            println!();
        }
        "See above".to_string()
    }
}
