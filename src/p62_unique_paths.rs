#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
// The robot can only move either down or right at any point in time.
// The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
// How many possible unique paths are there?

// answers
impl Solution {
    pub fn p62_unique_paths(m: i32, n: i32) -> i32 {
        // codes
        let n = n as usize;
        let m = m as usize;
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p62_unique_paths_t1() {
        assert_eq!(Solution::p62_unique_paths(3, 7), 28);
    }
}
