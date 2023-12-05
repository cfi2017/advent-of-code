use std::str::Bytes;
use itertools::Itertools;
use packed_simd::{FromCast, u8x16, u32x16};
use crate::aoc::Puzzle;
use crate::aoc::*;
use crate::aoc_boilerplate;

pub struct PuzzleDay;

const DFA: &[[u8; 14]] = &[
    //   0  1  2  3  4  5  6  7  8  9  10 11 12 13
    //   o  e  r  x  n  t  w  h  f  u  i  v  s  g
    [1, 5, 0, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 0
    [1, 5, 0, 0, 7, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 1
    [1, 5, 0, 0, 6, 2, 8, 9, 3, 0, 0, 0, 4, 0], // 2
    [10, 5, 0, 0, 6, 2, 0, 0, 3, 0, 11, 0, 4, 0], // 3
    [1, 13, 0, 0, 6, 2, 0, 0, 3, 0, 12, 0, 4, 0], // 4
    [1, 5, 0, 0, 6, 2, 0, 0, 3, 0, 14, 0, 4, 0], // 5
    [1, 5, 0, 0, 6, 2, 0, 0, 3, 0, 15, 0, 4, 0], // 6
    [1, 25, 0, 0, 6, 2, 0, 0, 3, 0, 15, 0, 4, 0], // 7
    [26, 5, 0, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 8
    [1, 5, 16, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 9
    [1, 5, 0, 0, 7, 2, 0, 0, 3, 17, 0, 0, 4, 0], // 10
    [1, 5, 0, 0, 6, 2, 0, 0, 3, 0, 0, 18, 4, 0], // 11
    [1, 5, 0, 30, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 12
    [1, 5, 0, 0, 6, 2, 0, 0, 3, 0, 14, 19, 4, 0], // 13
    [1, 5, 0, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 20],// 14
    [1, 5, 0, 0, 21, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 15
    [1, 22, 0, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 16
    [1, 5, 28, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 17
    [1, 29, 0, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 18
    [1, 23, 0, 0, 6, 2, 0, 0, 3, 0, 0, 0, 4, 0], // 19
    [1, 5, 0, 0, 6, 2, 0, 24, 3, 0, 0, 0, 4, 0], // 20
    [1, 33, 0, 0, 6, 2, 0, 0, 3, 0, 15, 0, 4, 0], // 21
    [1, 27, 0, 0, 6, 2, 0, 0, 3, 0, 14, 0, 4, 0], // 22
    [1, 5, 0, 0, 31, 2, 0, 0, 3, 0, 14, 0, 4, 0], // 23
    [1, 5, 0, 0, 6, 32, 0, 0, 3, 0, 0, 0, 4, 0], // 24
];


const DFA_REV: &[[u8; 14]] = &[
    //0  1  2  3  4  5  6  7  8  9  10 11 12 13 (token)
    //o  e  r  x  n  t  w  h  f  u  i  v  s  g
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], //  0 no match
    [2, 12, 3, 4, 10, 6, 0, 0, 0, 0, 0, 17, 0, 0], //  1 e
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], //  2 o
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 15, 0, 0, 0, 0], //  3 r
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 19, 0, 0, 0], //  4 x
    [2, 7, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], //  5 n
    [2, 1, 3, 4, 5, 6, 0, 20, 0, 0, 0, 0, 0, 0], //  6 t
    [2, 12, 3, 4, 5, 6, 0, 0, 0, 0, 0, 8, 0, 0], //  7 ne
    [2, 9, 3, 4, 5, 6, 0, 0, 0, 0, 18, 0, 0, 0], //  8 nev
    [2, 12, 3, 4, 31, 6, 0, 0, 0, 0, 0, 0, 0, 0], //  9 neve
    [25, 1, 3, 4, 5, 6, 0, 0, 0, 0, 11, 0, 0, 0], // 10 en
    [2, 1, 3, 4, 33, 6, 0, 0, 0, 0, 0, 0, 0, 0], // 11 eni
    [2, 12, 13, 4, 10, 6, 0, 0, 0, 0, 0, 17, 0, 0], // 12 ee
    [2, 1, 3, 4, 5, 6, 0, 14, 0, 0, 0, 0, 0, 0], // 13 eer
    [2, 1, 3, 4, 5, 27, 0, 0, 0, 0, 0, 0, 0, 0], // 14 eerh
    [16, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], // 15 ru
    [2, 1, 3, 4, 5, 6, 0, 0, 28, 0, 0, 0, 0, 0], // 16 ruo
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], // 17 ev
    [2, 1, 3, 4, 5, 6, 0, 0, 29, 0, 18, 0, 0, 0], // 18 evi
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 30, 0], // 19 xi
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 21], // 20 th
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 22, 0, 0, 0], // 21 thg
    [2, 32, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], // 22 thgi
    // unused states :)
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], // 23
    [2, 1, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0], // 24
];

// fake emit states are 25-33
const EMIT_START: u8 = 25;

//zyxwvutsrqponmlkjihgfedcba
const CHARSET_MASK: u32 = 0b00111111100110000111110000;
const TOKEN_TABLE: [u8; 26] = [
    //  a  b  c  d  e  f  g  h  i  j  k  l  m  n  o  p  q  r  s  t  u  v  w  x  y  z
    0, 0, 0, 0, 1, 8, 13, 7, 10, 0, 0, 0, 0, 4, 0, 0, 0, 2, 12, 5, 9, 11, 6, 3, 0, 0
];

fn parse_line(line: &String) -> Vec<i32> {
    line.chars().filter(|c| c.is_digit(10)).map(|c| (c as u8 - 48) as i32).collect::<Vec<i32>>()
}

fn simd_sum_from_iterator<I>(mut data_iter: I) -> u32
    where
        I: Iterator<Item=u8>,
{
    let simd_chunk_size = 16;
    let mut sum = 0u32;
    let mut buffer = [0u8; 16]; // Buffer for a SIMD chunk
    let mut buffer_index = 0;

    for value in data_iter {
        buffer[buffer_index] = value;
        buffer_index += 1;

        if buffer_index == simd_chunk_size {
            // Buffer is full, process it
            let simd_chunk = u8x16::from_slice_unaligned(&buffer);
            let simd_chunk = u32x16::from(simd_chunk);
            sum += simd_chunk.wrapping_sum();
            buffer_index = 0; // Reset buffer index
        }
    }

    // Handle the remainder
    for i in 0..buffer_index {
        sum += buffer[i] as u32;
    }

    sum
}

#[inline]
fn first_part(line: &[u8]) -> (u8, u8) {
    unsafe {
        let first = line.iter().find(|x| **x <= b'9').map(|c| c - b'0').unwrap_unchecked();
        let last = line.iter().rev().find(|x| **x <= b'9').map(|c| c - b'0').unwrap_unchecked();
        (first, last)
    }
}


fn first_num(input: impl Iterator<Item=u8>, table: &[[u8; 14]]) -> Option<u8> {
    let mut state = 0;
    for c in input {
        if c.is_ascii_digit() {
            return Some(c - b'0');
        }

        let char_idx = (c - b'a') as usize;
        if (1 << char_idx) & CHARSET_MASK == 0 {
            state = 0;
            continue;
        }
        let token = TOKEN_TABLE[char_idx] as usize;
        let next = table[state][token] as usize;

        if next >= EMIT_START as usize {
            let next = next as u8;
            return Some(next - EMIT_START + 1);
        } else {
            state = next;
        }
    }

    None
}


fn parse_line_complex(line: &[u8]) -> u8 {
    // first
    let first = first_num(line.iter().copied(), DFA).unwrap();
    let last = first_num(line.iter().rev().copied(), DFA_REV).unwrap();
    first * 10 + last
}

aoc_boilerplate!(2023, 1, sanitize_input, solve_a, solve_b);

pub fn sanitize_input(input: &str) -> impl Iterator<Item=&str> {
    input.split('\n')
        .filter(|x| !x.is_empty())
}

pub fn sanitize_input_a(input: &str) -> Bytes {
    input.bytes()
}


pub fn solve_a_alt(input: impl Iterator<Item=u8>) -> u32 {
    let mut sum = 0;
    let mut last = 0;
    let mut first = 0;
    for c in input {
        if c == b'\n' {
            sum += (first * 10 + last) as u32;
            first = 0;
            last = 0;
        } else if c <= b'9' {
            if first == 0 {
                first = c - b'0';
            }
            last = c - b'0';
        }
    }
    sum
}

pub fn solve_a(input: impl Iterator<Item=&str>) -> u32 {
    simd_sum_from_iterator(input
        .map(|x| x.as_bytes())
        .map(first_part)
        .map(|(a, b)| a * 10 + b))
        // .map(|x| x as u32)
        // .sum()
    // .fold(0_i32, |sum, x| sum + x as i32)
}

pub fn solve_b(input: impl Iterator<Item=&str>) -> u32 {
    simd_sum_from_iterator(input
        .map(|x| x.as_bytes())
        .map(parse_line_complex))
        // .map(|x| x as u32)
        // .sum()
}


#[cfg(test)]
mod tests {
    use crate::add_test;
    use super::*;

    add_test!(test_solve_a_example, solve_a, r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#, 142);

    add_test!(test_solve_b_example, solve_b, r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#, 281);
}
