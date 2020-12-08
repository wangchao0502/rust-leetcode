#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
// Return the array in the form [x1,y1,x2,y2,...,xn,yn].

// answers
impl Solution {
    pub fn p1470_shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // codes
        let mut ans = vec![];
        let n = n as usize;

        for i in 0..n {
            ans.push(nums[i]);
            ans.push(nums[n + i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1470_shuffle_t1() {
        assert_eq!(
            Solution::p1470_shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
    }

    #[test]
    fn p1470_shuffle_t2() {
        assert_eq!(
            Solution::p1470_shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
    }
}
