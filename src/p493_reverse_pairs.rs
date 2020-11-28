#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an array nums, we call (i, j) an important reverse pair if i < j and nums[i] > 2*nums[j].
// You need to return the number of important reverse pairs in the given array.

// answers
// ! Link -> 327
// ! merge sort
impl Solution {
    fn merge_sort(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        if left == right {
            0
        } else {
            let mid = (left + right) / 2;
            let n1 = Self::merge_sort(nums, left, mid);
            let n2 = Self::merge_sort(nums, mid + 1, right);
            let mut ret = n1 + n2;

            // 首先统计下标对的数量
            let mut i = left;
            let mut j = mid + 1;
            while i <= mid {
                while j <= right && nums[i] as i64 > 2 * nums[j] as i64 {
                    j += 1;
                }
                ret += j - mid - 1;
                i += 1;
            }

            // 随后合并两个排序数组
            let mut sorted = vec![0; right - left + 1];
            let mut p1 = left;
            let mut p2 = mid + 1;
            let mut p = 0;

            while p1 <= mid || p2 <= right {
                if p1 > mid {
                    sorted[p] = nums[p2];
                    p2 += 1;
                } else if p2 > right {
                    sorted[p] = nums[p1];
                    p1 += 1;
                } else {
                    if nums[p1] < nums[p2] {
                        sorted[p] = nums[p1];
                        p1 += 1;
                    } else {
                        sorted[p] = nums[p2];
                        p2 += 1;
                    }
                }
                p += 1;
            }
            for i in 0..sorted.len() {
                nums[left + i] = sorted[i];
            }
            ret
        }
    }

    pub fn p493_reverse_pairs(mut nums: Vec<i32>) -> i32 {
        // codes
        if nums.is_empty() {
            0
        } else {
            let len = nums.len();
            Self::merge_sort(&mut nums, 0, len - 1) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p493_reverse_pairs_t1() {
        assert_eq!(Solution::p493_reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    }

    #[test]
    fn p493_reverse_pairs_t2() {
        assert_eq!(Solution::p493_reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    }
}
