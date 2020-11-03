#![allow(dead_code)]

// use mods

pub struct Solution {}

// Given a matrix A, return the transpose of A.
// The transpose of a matrix is the matrix flipped over it's main diagonal, switching the row and column indices of the matrix.

// answers
impl Solution {
    pub fn p867_transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if a.len() == 0 {
            return vec![];
        }

        let mut ans = vec![vec![]; a[0].len()];

        for i in 0..a.len() {
            for j in 0..a[i].len() {
                ans[j].push(a[i][j]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p867_transpose_t1() {
        assert_eq!(
            Solution::p867_transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }
}
