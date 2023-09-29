use itertools::Itertools;

use crate::aoc::Puzzle;
use crate::e2019::vec3::Vec3;

pub struct Day08;

type BASE = i32;

impl Puzzle<Vec3<BASE>, BASE, String, 2019, 8> for Day08 {
    fn sanitize_input(&self, input: &str) -> Vec3<BASE> {
        input.chars()
            .flat_map(|c| c.to_digit(10))
            .map(|c| c as BASE)
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

    fn solve_a(&self, input: Vec3<BASE>) -> BASE {
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
        (ones * twos) as BASE
    }

    fn solve_b(&self, input: Vec3<BASE>) -> String {
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
