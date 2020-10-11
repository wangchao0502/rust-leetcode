#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// This dp problem is hard
// following code is optimized
impl Solution {
    pub fn p416_can_partition(nums: Vec<i32>) -> bool {
        // codes
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let capacity = sum / 2;
        let mut dp = vec![false; capacity as usize + 1];
        dp[0] = true;
        for i in 0..n {
            for j in (0..=capacity).rev() {
                if j - nums[i] >= 0 {
                    let j = j as usize;
                    dp[j] = dp[j] || dp[j - nums[i] as usize];
                }
            }
        }
        dp[capacity as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = vec![1, 5, 11, 5];
        let result = Solution::p416_can_partition(input);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let input = vec![1, 2, 3, 5];
        let result = Solution::p416_can_partition(input);
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let input = vec![2, 2, 3, 5];
        let result = Solution::p416_can_partition(input);
        assert_eq!(result, false);
    }

    #[test]
    fn t4() {
        let input = vec![1, 2, 3, 4, 5, 6, 7];
        let result = Solution::p416_can_partition(input);
        assert_eq!(result, true);
    }

    #[test]
    fn t5() {
        let input = vec![14, 9, 8, 4, 3, 2];
        let result = Solution::p416_can_partition(input);
        assert_eq!(result, true);
    }

    #[test]
    fn t6() {
        let input = vec![
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 99, 97,
        ];
        let result = Solution::p416_can_partition(input);
        assert_eq!(result, false);
    }
}
