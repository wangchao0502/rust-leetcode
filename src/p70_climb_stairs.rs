#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
// 注意：给定 n 是一个正整数。

// answers
impl Solution {
    pub fn p70_climb_stairs(n: i32) -> i32 {
        // codes
        // dp[i]表示去第i层有多少种方法
        let mut dp = vec![0; (n + 2) as usize];

        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 2;

        if n < 3 {
            return dp[n as usize];
        }

        for i in 3..=n as usize {
            dp[i] = dp[i - 2] + dp[i - 1];
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p70_climb_stairs_t1() {
        assert_eq!(Solution::p70_climb_stairs(2), 2);
    }

    #[test]
    fn p70_climb_stairs_t2() {
        assert_eq!(Solution::p70_climb_stairs(3), 3);
    }
}
