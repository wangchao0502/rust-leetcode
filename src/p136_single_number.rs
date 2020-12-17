#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

// answers
impl Solution {
    pub fn p136_single_number(nums: Vec<i32>) -> i32 {
        // codes
        let mut ans = 0;

        for n in nums {
            // 相同就消掉
            ans ^= n;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p136_single_number_t1() {
        assert_eq!(Solution::p136_single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn p136_single_number_t2() {
        assert_eq!(Solution::p136_single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
