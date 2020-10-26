#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// backtrack
impl Solution {
    fn render(board: &Vec<usize>, size: usize) -> Vec<String> {
        board
            .iter()
            .map(|&i| ".".repeat(i) + "Q" + &".".repeat(size - i - 1))
            .collect()
    }

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

    fn backtrack(board: &mut Vec<usize>, size: usize, level: usize, res: &mut Vec<Vec<String>>) {
        if level >= size {
            res.push(Self::render(board, size));
            return;
        }

        for p in 0..size {
            if Self::check(&board[..], (level, p)) {
                board[level] = p;
                Self::backtrack(board, size, level + 1, res);
            }
        }
    }

    pub fn p51_solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // codes
        let mut board = vec![0; n as usize];
        let mut res = vec![];

        Self::backtrack(&mut board, n as usize, 0, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p51_solve_n_queens_t1() {
        assert_eq!(
            Solution::p51_solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }
}
