#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// Given an array nums of n integers and an integer target, are there elements a, b, c,
// and d in nums such that a + b + c + d = target?
// Find all unique quadruplets in the array which gives the sum of target.
// Notice that the solution set must not contain duplicate quadruplets.
impl Solution {
    pub fn p18_four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // codes
        if nums.len() < 4 {
            return vec![];
        }

        let mut nums = nums.clone();
        let mut ans = vec![];

        nums.sort();

        // jump can remove repeat result
        for i in 0..(nums.len() - 3) {
            // jump same number
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in (i + 1)..(nums.len() - 2) {
                // jump same number
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let rest = target - nums[i] - nums[j];
                let mut lo = j + 1;
                let mut hi = nums.len() - 1;

                // promote
                let min2sum = nums[lo] + nums[lo + 1];
                let max2sum = nums[hi] + nums[hi - 1];
                if min2sum > rest || max2sum < rest {
                    continue;
                }

                while lo < hi {
                    // jump same number
                    if lo > j + 1 && nums[lo] == nums[lo - 1] {
                        lo += 1;
                        continue;
                    }
                    if hi < nums.len() - 1 && nums[hi] == nums[hi + 1] {
                        hi -= 1;
                        continue;
                    }

                    let sum = nums[lo] + nums[hi];

                    if sum > rest {
                        hi -= 1;
                    } else if sum < rest {
                        lo += 1;
                    } else {
                        ans.push(vec![nums[i], nums[j], nums[lo], nums[hi]]);
                        lo += 1;
                        hi -= 1;
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

    #[test]
    fn p18_four_sum_t1() {
        // [-2, -1, 0, 0, 1, 2]
        assert_eq!(
            Solution::p18_four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn p18_four_sum_t2() {
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::p18_four_sum(vec![], 0), ans);
    }

    #[test]
    fn p18_four_sum_t3() {
        assert_eq!(
            Solution::p18_four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]]
        );
    }
}
