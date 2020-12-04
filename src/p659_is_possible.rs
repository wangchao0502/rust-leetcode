#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an array nums sorted in ascending order, return true if and only if you can split it into 1 or
// more subsequences such that each subsequence consists of consecutive integers and has length at least 3.

// answers
// https://leetcode-cn.com/problems/split-array-into-consecutive-subsequences/solution/tan-xin-o1-kong-jian-fu-za-du-de-zui-you-jie-fa-by
impl Solution {
    pub fn p659_is_possible(nums: Vec<i32>) -> bool {
        // codes
        let n = nums.len();
        let mut dp1 = 0;
        let mut dp2 = 0;
        let mut dp3 = 0;
        let mut i: usize = 0;

        while i < n {
            let start = i;
            let x = nums[i];

            while i < n && nums[i] == x {
                i += 1;
            }

            let cnt = i - start;

            if start > 0 && x != nums[start - 1] + 1 {
                if dp1 + dp2 > 0 {
                    return false;
                } else {
                    dp1 = cnt;
                    dp2 = 0;
                    dp3 = 0;
                }
            } else {
                if dp1 + dp2 > cnt {
                    return false;
                }
                let left = cnt - dp1 - dp2;
                let keep = dp3.min(left);
                dp3 = keep + dp2;
                dp2 = dp1;
                dp1 = left - keep;
            }
        }

        dp1 == 0 && dp2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p659_is_possible_t1() {
        assert_eq!(Solution::p659_is_possible(vec![1, 2, 3, 3, 4, 5]), true);
    }

    #[test]
    fn p659_is_possible_t2() {
        assert_eq!(
            Solution::p659_is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]),
            true
        );
    }
}
