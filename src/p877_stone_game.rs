#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 亚历克斯和李用几堆石子在做游戏。偶数堆石子排成一行，每堆都有正整数颗石子 piles[i] 。
// 游戏以谁手中的石子最多来决出胜负。石子的总数是奇数，所以没有平局。
// 亚历克斯和李轮流进行，亚历克斯先开始。 每回合，玩家从行的开始或结束处取走整堆石头。
// 这种情况一直持续到没有更多的石子堆为止，此时手中石子最多的玩家获胜。
// 假设亚历克斯和李都发挥出最佳水平，当亚历克斯赢得比赛时返回 true ，当李赢得比赛时返回 false 。

// answers
impl Solution {
    pub fn p877_stone_game(piles: Vec<i32>) -> bool {
        // codes
        // true
        let len = piles.len();
        // dp[i][j] 表示当剩下的石子堆为下标 i 到下标 j 时，当前玩家与另一个玩家的石子数量之差的最大值
        // 需要先理解3维再想这个二维就比较容易，再将二维优化到一维
        let mut dp = vec![vec![0; len]; len];

        for i in 0..len {
            dp[i][i] = piles[i];
        }

        for i in (0..len - 1).rev() {
            for j in i + 1..len {
                dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
            }
        }

        dp[0][len - 1] > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p877_stone_game_t1() {
        assert_eq!(Solution::p877_stone_game(vec![5, 3, 4, 5]), true);
    }
}
