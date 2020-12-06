#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.

// answers
impl Solution {
    pub fn p118_generate(num_rows: i32) -> Vec<Vec<i32>> {
        // codes
        let mut ans: Vec<Vec<i32>> = vec![];
        let num_rows = num_rows as usize;

        for i in 0..num_rows {
            let mut line = vec![1; i + 1];
            if i >= 2 {
                for j in 1..i {
                    line[j] = ans[i - 1][j - 1] + ans[i - 1][j];
                }
            }
            ans.push(line);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p118_generate_t1() {
        assert_eq!(
            Solution::p118_generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
