#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an array of integers nums sorted in ascending order,
// find the starting and ending position of a given target value.
// If target is not found in the array, return [-1, -1].
// Follow up: Could you write an algorithm with O(log n) runtime complexity?

// answers
// binary search
impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32, lower: bool) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut ans = nums.len() as i32;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] > target || (lower && nums[mid as usize] >= target) {
                right = mid - 1;
                ans = mid;
            } else {
                left = mid + 1;
            }
        }

        ans
    }

    pub fn p34_search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // codes
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let left_idx = Self::binary_search(&nums, target, true);
        let right_idx = Self::binary_search(&nums, target, false) - 1;
        if left_idx <= right_idx
            && right_idx < nums.len() as i32
            && nums[left_idx as usize] == target
            && nums[right_idx as usize] == target
        {
            vec![left_idx, right_idx]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p34_search_range_t1() {
        assert_eq!(
            Solution::p34_search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn p34_search_range_t2() {
        assert_eq!(
            Solution::p34_search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn p34_search_range_t3() {
        assert_eq!(Solution::p34_search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    fn p34_search_range_t4() {
        assert_eq!(Solution::p34_search_range(vec![1], 0), vec![-1, -1]);
    }
}
