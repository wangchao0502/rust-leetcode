#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定正整数 n，找到若干个完全平方数（比如 1, 4, 9, 16, ...）使得它们的和等于 n。
// 你需要让组成和的完全平方数的个数最少。

// answers
impl Solution {
    pub fn p279_num_squares(n: i32) -> i32 {
        // codes
        let mut dp = vec![0; n as usize + 1];
        for i in 1..=n {
            // 初始为最坏情况
            dp[i as usize] = i;
            let mut j = 1;
            while i - j * j >= 0 {
                dp[i as usize] = dp[i as usize].min(dp[(i - j * j) as usize] + 1);
                j += 1;
            }
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p279_num_squares_t1() {
        assert_eq!(Solution::p279_num_squares(12), 3);
    }

    #[test]
    fn p279_num_squares_t2() {
        assert_eq!(Solution::p279_num_squares(13), 2);
    }
}
