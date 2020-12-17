#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

// answers
impl Solution {
    pub fn p53_max_sub_array(nums: Vec<i32>) -> i32 {
        // codes
        // 这种代码可以兼容全负数情况，而不需要扫描全部列表
        let mut ans = nums[0];
        let mut sum = 0;

        for num in nums {
            if sum > 0 {
                sum += num;
            } else {
                sum = num;
            }
            ans = ans.max(sum);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p53_max_sub_array_t1() {
        assert_eq!(
            Solution::p53_max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn p53_max_sub_array_t2() {
        assert_eq!(Solution::p53_max_sub_array(vec![-1]), -1);
    }
}
