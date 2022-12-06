use std::fs;

// Parse the input file into a vector of instructions and the initial stacks of crates.
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut lines = input.trim().lines();
    let stacks = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|stack| {
            stack.chars().skip(1).take(stack.len() - 2).collect()
        })
        .collect();
    let instructions = lines
        .map(|line: &str| {
            let mut words = line.split_ascii_whitespace();
            let n: &str = words.nth(1).unwrap().parse().unwrap();
            let from = words.nth(2).unwrap().parse().unwrap();
            let _ = words.next().unwrap();
            let to = words.next().unwrap().parse().unwrap();
            (from, to)
        })
        .collect();
    (stacks, instructions)
}

// Apply a series of instructions to the given stacks of crates.
fn apply_instructions(stacks: &mut Vec<Vec<char>>, instructions: &[(usize, usize)]) {
    for &(from, to) in instructions {
        let crate_ = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(crate_);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file.");
    let (mut stacks, instructions) = parse_input(&input);

    // Apply the instructions to the stacks of crates.
    apply_instructions(&mut stacks, &instructions);

    // Print the top crate from each stack.
    for stack in &stacks {
        let top_crate = stack.last().unwrap();
        print!("{}", top_crate);
    }
    println!();
}
