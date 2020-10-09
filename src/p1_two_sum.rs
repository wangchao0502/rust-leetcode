#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p1_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // codes
        let (mut p1, mut p2, len) = (0, 1, nums.len());

        while p1 < len - 1 {
            while p2 < len {
                if nums[p1] + nums[p2] == target {
                    return vec![p1 as i32, p2 as i32];
                }
                p2 += 1;
            }
            p1 += 1;
            p2 = p1 + 1;
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = vec![2, 7, 1, 15];
        let output = vec![0, 1];
        let target = 9;

        let result = Solution::p1_two_sum(input, target);
        assert_eq!(output, result);
    }
}

