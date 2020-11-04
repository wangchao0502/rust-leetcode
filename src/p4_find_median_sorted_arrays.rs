#![allow(dead_code)]

// use mods

pub struct Solution {}

// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
// Follow up: The overall run time complexity should be O(log (m+n)).

// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106

// answers
// binary division
impl Solution {
    pub fn p4_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // codes
        if nums1.len() > nums2.len() {
            return Self::p4_find_median_sorted_arrays(nums2, nums1);
        }
        let (m, n) = (nums1.len(), nums2.len());
        let (mut left, mut right) = (0, m);
        let (mut median1, mut median2) = (0, 0);
        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;
            let nums_im1 = if i == 0 { std::i32::MIN } else { nums1[i - 1] };
            let nums_i = if i == m { std::i32::MAX } else { nums1[i] };
            let nums_jm1 = if j == 0 { std::i32::MIN } else { nums2[j - 1] };
            let nums_j = if j == n { std::i32::MAX } else { nums2[j] };
            if nums_im1 <= nums_j {
                median1 = nums_im1.max(nums_jm1);
                median2 = nums_i.min(nums_j);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }
        if (m + n) % 2 == 0 {
            (median1 + median2) as f64 / 2.0
        } else {
            median1 as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p4_find_median_sorted_arrays_t1() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t2() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t3() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 2], vec![]),
            1.5
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t4() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5]),
            3.0
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t5() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5]),
            3.0
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t6() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]),
            3.5
        );
    }
    #[test]
    fn p4_find_median_sorted_arrays_t7() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4, 6]),
            3.5
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t8() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8]),
            4.5
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t9() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 7], vec![2, 3, 4, 5, 6]),
            4.0
        );
    }

    #[test]
    fn p4_find_median_sorted_arrays_t10() {
        assert_eq!(
            Solution::p4_find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8, 9, 10]),
            5.5
        );
    }
}
