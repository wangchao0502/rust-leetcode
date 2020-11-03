#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p349_intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // codes
        use std::collections::HashSet;
        let m1: HashSet<_> = nums1.iter().cloned().collect();
        let m2: HashSet<_> = nums2.iter().cloned().collect();
        m1.intersection(&m2).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p349_intersection_t1() {
        assert_eq!(
            Solution::p349_intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
    }
}
