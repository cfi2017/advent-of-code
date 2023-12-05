
use crate::aoc::Puzzle;

struct Day04;

#[derive(Clone)]
struct Input {
    boards: Vec<Board>,
    sequence: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Board {
    rows: Vec<Vec<Option<u32>>>,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

impl Board {
    fn remove(&mut self, n: u32) {
        let mut x = 0;
        while x < self.rows.len() {
            let row = self.rows.get_mut(x).unwrap();
            let mut y = 0;
            while y < row.len() {
                let opt_num = row.get_mut(y).unwrap();
                if opt_num.is_some() && n == (*opt_num).unwrap() {
                    *opt_num = None;
                }
                y += 1;
            }
            x += 1;
        }
    }

    fn check(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|n| n.is_none())) ||
            transpose(self.rows.clone()).iter().any(|row| row.iter().all(|n| n.is_none()))
    }

    fn sum(&self) -> u32 {
        self.rows.iter().fold(0, |a, row| a + row.iter()
            .fold(0, |aa, b| {
                match b {
                    Some(n) => aa + n,
                    None => aa,
                }
            }))
    }
}

impl Input {
    fn from(input: &str) -> Input {
        let mut iter = input.trim().split('\n');
        // first element is sequence
        let sequence = iter.next().unwrap().split(',')
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();
        // skip the second line
        let _ = iter.next();

        let mut boards = Vec::new();
        let mut board = Board {
            rows: Vec::new()
        };
        for line in iter {
            if line.is_empty() {
                boards.push(board);
                board = Board {
                    rows: Vec::new()
                }
            } else {
                board.rows.push(line
                    .trim_start()
                    .replace("  ", " ")
                    .split(' ')
                    .map(str::parse::<u32>)
                    .map(Result::unwrap).map(Some).collect())
            }
        }
        boards.push(board);

        Input {
            boards,
            sequence,
        }
    }
}

impl Puzzle<Input, i32, i32, 2021, 4> for Day04 {

    fn sanitize_input(&self, input: &str) -> Input {
        Input::from(input)
    }

    fn solve_a(&self, input: Input) -> i32 {
        let mut boards = input.boards.clone();
        for s in input.sequence {
            for board in &mut boards {
                board.remove(s);
                // println!("board: {:?}", board);
                if board.check() {
                    return (board.sum() * s) as i32
                }
            }
        }
        panic!("no solution after entire input sequence");
    }

    fn solve_b(&self, input: Input) -> i32 {
        let mut boards = input.boards.clone();

        for s in input.sequence {
            let mut i = 0;
            while i < boards.len() {
                i += 1;
                let board = boards.get_mut(i - 1).unwrap();
                board.remove(s);
                if board.check() {
                    println!("board won: {}", i - 1);
                    if boards.len() == 1 {
                        return (boards.first().unwrap().sum() * s) as i32
                    }
                    // println!("board: {:?}", board);
                    boards.remove(i - 1);
                    i -= 1;
                }
            }
        }
        panic!("no solution after entire input sequence");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input = Input::from(input);
        // println!("{:?}", &input.boards);
        assert_eq!(Day04.solve_a(input.clone()), 4512);
        assert_eq!(Day04.solve_b(input.clone()), 1924);
    }

    #[test]
    fn board_check_complete_row() {
        let board = Board {
            rows: vec![
                vec![None, None, None],
                vec![Some(1), Some(2), Some(3)]
            ]
        };
        assert!(board.check());
    }

    #[test]
    fn board_check_complete_column() {
        let board = Board {
            rows: vec![
                vec![None, None, Some(4)],
                vec![Some(1), None, Some(3)]
            ]
        };
        assert!(board.check());
    }

    #[test]
    fn board_check_incomplete_board() {
        let board = Board {
            rows: vec![
                vec![None, Some(2), None],
                vec![Some(1), Some(3), Some(4)]
            ]
        };
        assert!(!board.check());
    }

    #[test]
    fn board_sum_row() {
        let board = Board {
            rows: vec![
                vec![None, Some(2), Some(3)],
                vec![None, Some(2), Some(3)],
                vec![None, Some(2), Some(3)],
            ]
        };
        assert_eq!(board.sum(), 15);
    }

    #[test]
    fn board_sum_column() {
        let board = Board {
            rows: vec![
                vec![Some(3), None],
                vec![Some(2), None]
            ]
        };
        assert_eq!(board.sum(), 5);
    }

    #[test]
    fn board_sum_multi() {
        let board = Board {
            rows: vec![
                vec![Some(3), Some(8)],
                vec![Some(2), Some(5)]
            ]
        };
        assert_eq!(board.sum(), 18);
    }
}
