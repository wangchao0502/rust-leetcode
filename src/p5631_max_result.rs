#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。
// 一开始你在下标 0 处。每一步，你最多可以往前跳 k 步，但你不能跳出数组的边界。
// 也就是说，你可以从下标 i 跳到 [i + 1， min(n - 1, i + k)] 包含 两个端点的任意位置。
// 你的目标是到达数组最后一个位置（下标为 n - 1 ），你的 得分 为经过的所有数字之和。
// 请你返回你能得到的 最大得分 。

// answers
// 定义 f[i]f[i] 表示到下标i处的最大得分，则 f[i] = max(f[i-k..i-1]) + nums[i]
impl Solution {
    pub fn p5631_max_result(nums: Vec<i32>, k: i32) -> i32 {
        // codes
        let k = k as usize;
        let mut queue = vec![0; 100010];
        let mut head = 1;
        let mut tail = 1;
        let mut dp = vec![0; 100010];

        let len = nums.len();
        dp[0] = nums[0];
        queue[1] = 0;

        for i in 1..len {
            while i > k && head <= tail && queue[head] < i - k {
                head += 1;
            }
            // queue[head] 表示 dp[i-k..i-1]的最大值
            dp[i] = dp[queue[head]] + nums[i];
            while head <= tail && dp[i] > dp[queue[tail]] {
                tail -= 1;
            }
            tail += 1;
            queue[tail] = i;
        }

        dp[len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5631_max_result_t1() {
        assert_eq!(Solution::p5631_max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
    }

    #[test]
    fn p5631_max_result_t2() {
        assert_eq!(Solution::p5631_max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
    }

    #[test]
    fn p5631_max_result_t3() {
        assert_eq!(
            Solution::p5631_max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2),
            0
        );
    }
}
