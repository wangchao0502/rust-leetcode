#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// dp -> 哨兵问题，最长上升子序列
// 对ages和scores排序
// dp[i] -> 选择第i个队员的最大分数
// i   ages    scores  max
// 0   1       5       5
// 1   1       5       10
// 2   2       4       4
// 3   2       6       16
// 转移方程 dp[i] = max{ dp[0..i - 1] if not(age < ages[i] and score > scores[i]) } + scores[i]
impl Solution {
    pub fn p5545_best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        // codes
        let mut dp = vec![0; scores.len()];
        let n = scores.len();
        let mut mp = vec![];
        let mut maxs = 0;

        for i in 0..n {
            mp.push(vec![ages[i], scores[i]]);
        }

        // sort mp by age and score
        mp.sort_by_key(|k| (k[0], k[1]));
        dp[0] = mp[0][1];

        for i in 1..n {
            for j in 0..i {
                if mp[j][1] <= mp[i][1] {
                    dp[i] = dp[i].max(dp[j]);
                }
            }
            dp[i] += mp[i][1];
            maxs = maxs.max(dp[i]);
        }

        maxs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5545_best_team_score_t1() {
        assert_eq!(
            Solution::p5545_best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]),
            16
        );
    }

    #[test]
    fn p5545_best_team_score_t2() {
        assert_eq!(
            Solution::p5545_best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]),
            6
        );
    }
}
