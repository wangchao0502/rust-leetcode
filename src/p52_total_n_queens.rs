#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    fn check(board: &[usize], queen_pos: (usize, usize)) -> bool {
        let (row, col) = queen_pos;
        for i in 0..row {
            if col == board[i]
                || (row as i32 - i as i32).abs() == (col as i32 - board[i] as i32).abs()
            {
                return false;
            }
        }
        true
    }

    fn backtrack(board: &mut Vec<usize>, size: usize, level: usize, count: &mut i32) {
        if level >= size {
            *count += 1;
            return;
        }

        for p in 0..size {
            if Self::check(&board[..], (level, p)) {
                board[level] = p;
                Self::backtrack(board, size, level + 1, count);
            }
        }
    }

    pub fn p52_total_n_queens(n: i32) -> i32 {
        // codes
        let mut board = vec![0; n as usize];
        let mut count = 0;

        Self::backtrack(&mut board, n as usize, 0, &mut count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p52_total_n_queens_t1() {
        assert_eq!(Solution::p52_total_n_queens(4), 2);
    }

    #[test]
    fn p52_total_n_queens_t2() {
        assert_eq!(Solution::p52_total_n_queens(8), 92);
    }
}
