#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// We have a two dimensional matrix A where each value is 0 or 1.
// A move consists of choosing any row or column, and toggling each value in that row or column:
// changing all 0s to 1s, and all 1s to 0s.
// After making any number of moves, every row of this matrix is interpreted as a binary number,
// and the score of the matrix is the sum of these numbers.
// Return the highest possible score.

// answers
// 第一步：将首列全部置位1，保证最高位全部取到，首列不为1的行全部翻转
// 第二步：从第二列开始，将所有列中1的数量小于0的数量的行翻转，保证取1的数量尽可能多
// 第三步：计算结果返回
impl Solution {
    pub fn p861_matrix_score(a: Vec<Vec<i32>>) -> i32 {
        // 实际不需要反转，只需要计算
        let row = a.len();
        let col = a[0].len();
        let mut ans = (row * (1 << (col - 1))) as i32;

        for j in 1..col {
            let mut ones = 0;
            for i in 0..row {
                if a[i][0] == 1 {
                    ones += a[i][j];
                } else {
                    // 如果这一行进行了行反转，则该元素的实际取值为 1 - a[i][j]
                    ones += 1 - a[i][j];
                }
            }
            let k = ones.max(row as i32 - ones);
            ans += k * (1 << (col - j - 1) as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p861_matrix_score_t1() {
        assert_eq!(
            Solution::p861_matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
    }
}
