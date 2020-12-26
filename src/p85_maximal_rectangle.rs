#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。

// answers
// 通俗思路，其实不是动态规划，dp数组用的不太准
impl Solution {
    pub fn p85_maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        // codes
        let row = matrix.len();

        if row == 0 {
            return 0;
        }

        let col = matrix[0].len();
        // 第三维数组 vec![0]表示横向连续个数，vec![1]表示纵向连续个数
        // dp[i][j]表示第i行，第j列。增加的外边框作为哨兵
        let mut dp = vec![vec![vec![0, 0]; col + 1]; row + 1];
        let mut ans = 0;

        for i in 1..=row {
            for j in 1..=col {
                if (matrix[i - 1][j - 1] as u8 - b'0') as i32 == 1 {
                    dp[i][j][0] = dp[i][j - 1][0] + 1;
                    dp[i][j][1] = dp[i - 1][j][1] + 1;
                    // println!("{},{} => {},{}", i, j, dp[i][j][0], dp[i][j][1]);

                    if dp[i][j][0] > 1 && dp[i][j][1] > 1 {
                        // 多行，多列，判断有多少行是没有中空的
                        // 情况一：以行为基准
                        for m in 2..=dp[i][j][0] {
                            let mut row_tmp = i - 1;
                            while row_tmp > 0 && dp[row_tmp][j][0] >= m {
                                row_tmp -= 1;
                            }
                            ans = ans.max(m * (i - row_tmp) as i32);
                        }
                        for n in 2..=dp[i][j][1] {
                            // 情况二：以列为基准
                            let mut col_tmp = j - 1;
                            while col_tmp > 0 && dp[i][col_tmp][1] >= n {
                                col_tmp -= 1;
                            }
                            ans = ans.max(n * (j - col_tmp) as i32);
                        }
    
                    } else {
                        // 一行或一列
                        ans = ans.max(dp[i][j][0] * dp[i][j][1]);
                    }
                } else {
                    // 终止连续
                    dp[i][j] = vec![0, 0];
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p85_maximal_rectangle_t1() {
        assert_eq!(
            Solution::p85_maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }

    #[test]
    fn p85_maximal_rectangle_t2() {
        assert_eq!(Solution::p85_maximal_rectangle(vec![]), 0)
    }

    #[test]
    fn p85_maximal_rectangle_t3() {
        assert_eq!(Solution::p85_maximal_rectangle(vec![vec!['1']]), 1)
    }

    #[test]
    fn p85_maximal_rectangle_t4() {
        assert_eq!(Solution::p85_maximal_rectangle(vec![vec!['0']]), 0)
    }

    #[test]
    fn p85_maximal_rectangle_t5() {
        assert_eq!(
            Solution::p85_maximal_rectangle(vec![
                vec!['0', '0', '0', '0', '0', '0', '1'],
                vec!['0', '0', '0', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1', '1', '1'],
                vec!['0', '0', '0', '1', '1', '1', '1']
            ]),
            9
        )
    }
}
