#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。
// 子序列是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。
// 例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。

// answers
impl Solution {
    pub fn p300_length_of_lis(nums: Vec<i32>) -> i32 {
        // codes
        let mut dp = vec![1; nums.len()];

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p300_length_of_lis_t1() {
        assert_eq!(
            Solution::p300_length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
    }
}
