#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金
// 影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
// 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。

// answers
impl Solution {
    pub fn p198_rob(nums: Vec<i32>) -> i32 {
        // codes
        // dp[i][0] i房间不偷的最高金额
        // dp[i][1] i房间偷的最高金额
        let len = nums.len();
        let mut dp = vec![vec![0; 2]; len];

        if len == 0 {
            return 0;
        }

        dp[0][0] = 0;
        dp[0][1] = nums[0];

        for i in 1..len {
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }

        dp[len - 1][0].max(dp[len - 1][1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p198_rob_t1() {
        assert_eq!(Solution::p198_rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn p198_rob_t2() {
        assert_eq!(Solution::p198_rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn p198_rob_t3() {
        assert_eq!(Solution::p198_rob(vec![]), 0);
    }
}
