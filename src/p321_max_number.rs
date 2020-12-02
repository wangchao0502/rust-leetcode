#![allow(dead_code)]

// use mods
use std::cmp::max;
use std::collections::VecDeque;

pub struct Solution {}

// problem description
// Given two arrays of length m and n with digits 0-9 representing two numbers.
// Create the maximum number of length k <= m + n from digits of the two.
// The relative order of the digits from the same array must be preserved.
// Return an array of the k digits.
// Note: You should try to optimize your time and space complexity.

// answers
impl Solution {
    // 1.从num1中取出i个数，从num2中取出k-i个数拼接成最终结果
    // 2.维护一个单调递减的栈，从num中依次往栈中添加或删除数据，直到栈中保留的数据量为期望值
    // 3.拼接过程类似归并排序，值大的取出来放在前面，值小的放在后面
    pub fn p321_max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for i in 0..=nums1.len() as i32 {
            if i <= k {
                if k - i <= nums2.len() as i32 {
                    let v1 = Solution::get_max_sub_array(nums1.as_ref(), i);
                    let v2 = Solution::get_max_sub_array(nums2.as_ref(), k - i);
                    let vv = Solution::merge_vec(v1.as_ref(), v2.as_ref());
                    result = max(result, vv);
                }
            }
        }
        return result;
    }

    // 使用单调递减的栈，获取指定个数的子数组
    fn get_max_sub_array(array: &Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: VecDeque<i32> = Default::default();
        let mut drop = array.len() - k as usize;
        for &digit in array {
            while (drop > 0) && (res.len() > 0) && (res[res.len() - 1] < digit) {
                res.pop_back();
                drop -= 1;
            }
            res.push_back(digit);
        }
        while drop > 0 {
            res.pop_back();
            drop -= 1;
        }
        return Vec::from(res);
    }

    // 合并列表
    fn merge_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
        let mut res: VecDeque<i32> = Default::default();
        let mut i = 0;
        let mut j = 0;
        let v1_len = v1.len();
        let v2_len = v2.len();
        while i < v1_len && j < v2_len {
            if v1[i..v1_len] > v2[j..v2_len] {
                res.push_back(v1[i]);
                i += 1;
            } else {
                res.push_back(v2[j]);
                j += 1;
            }
        }
        while i < v1_len {
            res.push_back(v1[i]);
            i += 1;
        }
        while j < v2_len {
            res.push_back(v2[j]);
            j += 1;
        }
        Vec::from(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p321_max_number_t1() {
        assert_eq!(
            Solution::p321_max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
    }

    #[test]
    fn p321_max_number_t2() {
        assert_eq!(
            Solution::p321_max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
    }
}
