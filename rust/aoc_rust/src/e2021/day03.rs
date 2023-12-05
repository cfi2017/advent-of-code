use crate::aoc::Puzzle;

#[derive(Clone)]
pub struct Input {
    bit_length: usize,
    nums: Vec<u32>,
}

fn bit_mask(bit_length: usize) -> u16 {
    2_u16.pow(bit_length as u32) - 1
}

fn invert(gamma: u16, bit_length: usize) -> u16 {
    (!gamma) & bit_mask(bit_length)
}

fn calc_gamma(v: &Vec<u32>, length: usize) -> u16 {
    let input_length = v.len() as u32;
    let mut counts = vec![0; length];
    for num in v {
        let mut bit_mask = 1;
        for i in &mut counts {
            *i += (num & bit_mask) / bit_mask;
            bit_mask <<= 1;
        }
    }

    let mut gamma: u16 = 0;
    for count in counts.iter().rev() {
        gamma <<= 1;
        if *count as f32 >= input_length as f32 / 2.0 {
            gamma += 1;
        }
    }
    gamma
}

pub struct Day03;
impl Day03 {

    fn gamma(&self, input: &Input) -> u16 {
        calc_gamma(&input.nums, input.bit_length)
    }
}

impl Puzzle<Input, i32, i32, 2021, 3> for Day03 {

    fn sanitize_input(&self, input: &str) -> Input {
        let bit_length = input.split('\n').nth(0).unwrap().chars().count();
        let nums = input.split('\n').filter(|s| !s.is_empty())
            .map(|s| u32::from_str_radix(s, 2))
            .map(Result::unwrap)
            .collect();
        Input {
            bit_length,
            nums,
        }
    }


    fn solve_a(&self, input: Input) -> i32 {
        let gamma = self.gamma(&input);
        i32::from(gamma * invert(gamma, input.bit_length))
    }

    fn solve_b(&self, input: Input) -> i32 {
        let gamma = self.gamma(&input);
        let epsilon = invert(gamma, input.bit_length);

        let mut gamma = gamma as u32;
        let mut epsilon = epsilon as u32;

        // let gamma = reverse_bits(gamma as i32) as u32;
        // let epsilon = reverse_bits(epsilon as i32) as u32;

        let mut result = input.nums.clone();
        let mut pos_mask = 2_u32.pow(input.bit_length as u32 - 1);
        for _ in 0..input.bit_length {
            result.retain(|n| {
                n & pos_mask == gamma & pos_mask
            });
            pos_mask >>= 1;
            gamma = calc_gamma(&result, input.bit_length) as u32;
            if result.len() == 1 {
                break
            }
        }

        let oxygen = *result.first().unwrap() as i32;

        let mut result = input.nums.clone();
        let mut pos_mask = 2_u32.pow(input.bit_length as u32 - 1);
        for _ in 0..input.bit_length {
            result.retain(|n| n & pos_mask == epsilon & pos_mask);
            pos_mask >>= 1;
            epsilon = invert(calc_gamma(&result, input.bit_length), input.bit_length) as u32;
            if result.len() == 1 {
                break
            }
        }
        let co2 = *result.first().unwrap() as i32;
        co2 * oxygen
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = Input {
            bit_length: 5,
            nums: [00100, 11110, 10110,
                10111, 10101, 01111,
                00111, 11100, 10000,
                11001, 00010, 01010].iter().map(|i| format!("{:05}", i))
                .map(|s| u32::from_str_radix(s.as_str(), 2))
                .map(Result::unwrap)
                .collect()
        };

        assert_eq!(Day03.solve_a(input.clone()), 198);
        assert_eq!(Day03.solve_b(input.clone()), 230);
    }

}
