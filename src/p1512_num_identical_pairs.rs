#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an array of integers nums.
// A pair (i,j) is called good if nums[i] == nums[j] and i < j.
// Return the number of good pairs.

// answers
// 组合数学
impl Solution {
    pub fn p1512_num_identical_pairs(nums: Vec<i32>) -> i32 {
        // codes
        let (mut count, mut arr) = (0, vec![0; 101]);
        nums.iter().for_each(|&x| {
            count += arr[x as usize];
            arr[x as usize] += 1;
        });
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1512_num_identical_pairs_t1() {
        assert_eq!(
            Solution::p1512_num_identical_pairs(vec![1, 2, 3, 1, 1, 3]),
            4
        );
    }
}
