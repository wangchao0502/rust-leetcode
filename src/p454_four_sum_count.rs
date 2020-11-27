#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given four lists A, B, C, D of integer values, compute how many tuples (i, j, k, l)
// there are such that A[i] + B[j] + C[k] + D[l] is zero.

// To make problem a bit easier, all A, B, C, D have same length of N where 0 ≤ N ≤ 500.
// All integers are in the range of -2^28 to 2^28 - 1 and the result is guaranteed to be at most 2^31 - 1.

// answers
impl Solution {
    pub fn p454_four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        // codes
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut ans = 0;

        // a + b => map
        for i in &a {
            for j in &b {
                *map.entry(i + j).or_insert(0) += 1;
            }
        }

        // c + d
        for i in &c {
            for j in &d {
                if let Some(count) = map.get(&-(i + j)) {
                    ans += count;
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
    fn p454_four_sum_count_t1() {
        assert_eq!(
            Solution::p454_four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
    }
}
