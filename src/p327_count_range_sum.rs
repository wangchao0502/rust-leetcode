#![allow(dead_code)]

// use mods

pub struct Solution {}

// Given an integer array nums, return the number of range sums that lie in [lower, upper] inclusive.
// Range sum S(i, j) is defined as the sum of the elements in nums between indices i and j (i ≤ j), inclusive.
// Note:
// A naive algorithm of O(n2) is trivial. You MUST do better than that.

// answers
impl Solution {
    pub fn p327_count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // codes
        if nums.is_empty() {
            return 0;
        }

        let mut sum = vec![nums[0] as i64; nums.len()];
        let mut count = 0;
        let lower = lower as i64;
        let upper = upper as i64;

        for i in 1..nums.len() {
            sum[i] = nums[i] as i64 + sum[i - 1];
        }

        for i in 0..nums.len() {
            if sum[i] >= lower && sum[i] <= upper {
                count += 1;
            }
            for j in 0..i {
                if sum[i] - sum[j] >= lower && sum[i] - sum[j] <= upper {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p327_count_range_sum_t1() {
        assert_eq!(Solution::p327_count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    }

    #[test]
    fn p327_count_range_sum_t2() {
        assert_eq!(Solution::p327_count_range_sum(vec![1, -1], 0, 0), 1);
    }

    #[test]
    fn p327_count_range_sum_t3() {
        assert_eq!(
            Solution::p327_count_range_sum(vec![2147483647, -2147483648, -1, 0], -1, 0),
            4
        );
    }
}
