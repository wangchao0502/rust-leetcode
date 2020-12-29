#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个已排序的正整数数组 nums，和一个正整数 n 。
// 从 [1, n] 区间内选取任意个数字补充到 nums 中，使得 [1, n] 区间内的任何数字都可以用 nums 中某几个数字的和来表示。
// 请输出满足上述要求的最少需要补充的数字个数。

// answers
// hard 数学，贪心
impl Solution {
    pub fn p330_min_patches(nums: Vec<i32>, n: i32) -> i32 {
        // codes
        let mut patches = 0;
        let mut x: i64 = 1;
        let len = nums.len();
        let mut index = 0;

        while x <= n as i64 {
            if index < len && nums[index] as i64 <= x {
                x += nums[index] as i64;
                index += 1;
            } else {
                x *= 2;
                patches += 1;
            }
        }

        patches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p330_min_patches_t1() {
        assert_eq!(Solution::p330_min_patches(vec![1, 3], 6), 1);
    }

    #[test]
    fn p330_min_patches_t2() {
        assert_eq!(Solution::p330_min_patches(vec![1, 5, 10], 20), 2);
    }
}
