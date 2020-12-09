#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an integer n and an integer start.
// Define an array nums where nums[i] = start + 2*i (0-indexed) and n == nums.length.
// Return the bitwise XOR of all elements of nums.

// answers
impl Solution {
    pub fn p1486_xor_operation(n: i32, start: i32) -> i32 {
        // codes
        let mut ans = start;
        for i in 1..n {
            ans ^= start + i * 2;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1486_xor_operation_t1() {
        // Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
        assert_eq!(Solution::p1486_xor_operation(5, 0), 8);
    }
}
