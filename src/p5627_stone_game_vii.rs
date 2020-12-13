#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 石子游戏中，爱丽丝和鲍勃轮流进行自己的回合，爱丽丝先开始 。
// 有 n 块石子排成一排。每个玩家的回合中，可以从行中 移除 最左边的石头或最右边的石头，
// 并获得与该行中剩余石头值之 和 相等的得分。当没有石头可移除时，得分较高者获胜。
// 鲍勃发现他总是输掉游戏（可怜的鲍勃，他总是输），所以他决定尽力 减小得分的差值 。爱丽丝的目标是最大限度地 扩大得分的差值 。
// 给你一个整数数组 stones ，其中 stones[i] 表示 从左边开始 的第 i 个石头的值，如果爱丽丝和鲍勃都 发挥出最佳水平 ，请返回他们 得分的差值 。

// 输入：stones = [5,3,1,4,2]
// 输出：6
// 解释：
// - 爱丽丝移除 2 ，得分 5 + 3 + 1 + 4 = 13 。游戏情况：爱丽丝 = 13 ，鲍勃 = 0 ，石子 = [5,3,1,4] 。
// - 鲍勃移除 5 ，得分 3 + 1 + 4 = 8 。游戏情况：爱丽丝 = 13 ，鲍勃 = 8 ，石子 = [3,1,4] 。
// - 爱丽丝移除 3 ，得分 1 + 4 = 5 。游戏情况：爱丽丝 = 18 ，鲍勃 = 8 ，石子 = [1,4] 。
// - 鲍勃移除 1 ，得分 4 。游戏情况：爱丽丝 = 18 ，鲍勃 = 12 ，石子 = [4] 。
// - 爱丽丝移除 4 ，得分 0 。游戏情况：爱丽丝 = 18 ，鲍勃 = 12 ，石子 = [] 。
// 得分的差值 18 - 12 = 6 。

// answers
// dp
// TODO 理解一下
impl Solution {
    pub fn p5627_stone_game_vii(stones: Vec<i32>) -> i32 {
        // codes
        let n = stones.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];

        for i in 1..=n {
            dp[i][i] = 0;
        }

        let mut sum = vec![0];

        for stone in stones {
            sum.push(sum.last().unwrap() + stone);
        }

        for i in 2..=n {
            for j in 1..(n - i + 2) {
                let r = i + j - 1;
                dp[j][r] = i32::max(
                    sum[r] - sum[j] - dp[j + 1][r],
                    sum[r - 1] - sum[j - 1] - dp[j][r - 1],
                );
            }
        }

        dp[1][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5627_stone_game_vii_t1() {
        assert_eq!(
            Solution::p5627_stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]),
            122
        );
    }

    #[test]
    fn p5627_stone_game_vii_t2() {
        assert_eq!(Solution::p5627_stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
    }
}
