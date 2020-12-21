#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 数组的每个索引作为一个阶梯，第 i个阶梯对应着一个非负数的体力花费值 cost[i](索引从0开始)。
// 每当你爬上一个阶梯你都要花费对应的体力花费值，然后你可以选择继续爬一个阶梯或者爬两个阶梯。
// 您需要找到达到楼层顶部的最低花费。在开始时，你可以选择从索引为 0 或 1 的元素作为初始阶梯。

// answers
impl Solution {
    pub fn p746_min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // codes
        let len = cost.len();
        let mut dp = vec![0; len];
        dp[0] = cost[0];
        dp[1] = cost[1];

        for i in 2..len {
            dp[i] = dp[i - 2].min(dp[i - 1]) + cost[i];
        }

        dp[len - 1].min(dp[len - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p746_min_cost_climbing_stairs_t1() {
        assert_eq!(
            Solution::p746_min_cost_climbing_stairs(vec![10, 15, 20]),
            15
        );
    }

    #[test]
    fn p746_min_cost_climbing_stairs_t2() {
        assert_eq!(
            Solution::p746_min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
