#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，
// 使得它们的和与 target 最接近。返回这三个数的和。假定每组输入只存在唯一答案。

// answers
impl Solution {
    pub fn p16_three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // codes
        nums.sort();

        let len = nums.len();
        let mut ans = nums[0] + nums[1] + nums[2];

        for i in 0..len {
            let mut l = i + 1;
            let mut r = len - 1;

            // -4, -1, 1, 2
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }
                if sum > target {
                    r -= 1;
                } else if sum < target {
                    l += 1;
                } else {
                    return target;
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
    fn p16_three_sum_closest_t1() {
        assert_eq!(Solution::p16_three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}
