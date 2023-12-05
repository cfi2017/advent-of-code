use crate::aoc::Puzzle;

pub struct Day01 {}
impl Puzzle<Vec<i32>, i32, i32, 2021, 1> for Day01 {

    fn sanitize_input(&self, input: &str) -> Vec<i32> {
        input.split('\n')
            .filter(|s| !s.is_empty())
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect()
    }

    fn solve_a(&self, input: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 1..input.len() {
            if input[i - 1] < input[i] {
                count += 1;
            }
        }

        count
    }

    fn solve_b(&self, input: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 3..input.len() {
            if input[i-3] + input[i-2] + input[i-1] < input[i-2] + input[i-1] + input[i] {
                count += 1
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let day = Day01{};
        let a = day.solve_a(test_input.clone());
        let b = day.solve_b(test_input.clone());
        assert_eq!(a, 7);
        assert_eq!(b, 5);
    }

}


