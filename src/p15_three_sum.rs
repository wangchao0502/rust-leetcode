#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，
// 使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
// 注意：答案中不可以包含重复的三元组。

// answers
// 首先对数组进行排序，排序后固定一个数 nums[i]，再使用左右指针指向 nums[i]后面的两端，
// 数字分别为 nums[L] 和 nums[R]，计算三个数的和 sum 判断是否满足为 0，满足则添加进结果集
// 如果 nums[i]大于 0，则三数之和必然无法等于 0，结束循环
// 如果 nums[i] == nums[i-1]，则说明该数字重复，会导致结果重复，所以应该跳过
// 当 sum == 0 时，nums[L] == nums[L+1] 则会导致结果重复，应该跳过，L++
// 当 sum == 0 时，nums[R] == nums[R−1] 则会导致结果重复，应该跳过，R--
// 时间复杂度：O(n^2)，n 为数组长度
impl Solution {
    pub fn p15_three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // codes
        let len = nums.len();
        let mut ans = vec![];
        nums.sort();

        for i in 0..len {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for left in i + 1..len {
                if nums[left] > -nums[i] || (nums[left] == -nums[i] && nums[left] != 0) {
                    break;
                }
                let target = -nums[i] - nums[left];
                if left == i + 1 || nums[left] != nums[left - 1] {
                    // 二分查找
                    let mut l = left + 1;
                    let mut r = len - 1;

                    while l <= r {
                        let mid = l + ((r - l) >> 2);
                        if nums[mid] > target {
                            r = mid - 1;
                        } else if nums[mid] < target {
                            l = mid + 1;
                        } else {
                            ans.push(vec![nums[i], nums[left], nums[mid]]);
                            break;
                        }
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::assert_eq_sorted;

    #[test]
    fn p15_three_sum_t1() {
        assert_eq_sorted!(
            Solution::p15_three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        );
    }
}
