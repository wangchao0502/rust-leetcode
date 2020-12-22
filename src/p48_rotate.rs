#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个 n × n 的二维矩阵表示一个图像。
// 将图像顺时针旋转 90 度。
// 说明：
// 你必须在原地旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要使用另一个矩阵来旋转图像。

// answers
impl Solution {
    pub fn p48_rotate(matrix: &mut Vec<Vec<i32>>) {
        // codes
        let n = matrix.len();

        for i in 0..(n / 2) {
            for j in 0..(n + 1) / 2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p48_rotate_t1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::p48_rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
