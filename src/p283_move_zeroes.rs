#![allow(dead_code)]

// use mods

pub struct Solution {}

// Given an array nums, write a function to move all 0's to the end of it
// while maintaining the relative order of the non-zero elements.

// answers
impl Solution {
    pub fn p283_move_zeroes(nums: &mut Vec<i32>) {
        // codes
        let (mut slow, mut fast) = (0, 0);
        // [slow, fast]之间全是0
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }
            fast += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p283_move_zeroes_t1() {
        let mut input = vec![0, 1, 0, 3, 12];
        Solution::p283_move_zeroes(&mut input);
        assert_eq!(input, vec![1, 3, 12, 0, 0]);
    }
}
