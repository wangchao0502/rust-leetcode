#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
// Return the running sum of nums.

// answers
impl Solution {
    pub fn p1480_running_sum(nums: Vec<i32>) -> Vec<i32> {
        // codes
        let mut ans = vec![nums[0]];

        for i in 1..nums.len() {
            ans.push(ans[i - 1] + nums[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1480_running_sum_t1() {
        assert_eq!(
            Solution::p1480_running_sum(vec![1, 2, 3, 4]),
            vec![1, 3, 6, 10]
        );
    }

    #[test]
    fn p1480_running_sum_t2() {
        assert_eq!(
            Solution::p1480_running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }
}
