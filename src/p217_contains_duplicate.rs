#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个整数数组，判断是否存在重复元素。
// 如果任意一值在数组中出现至少两次，函数返回 true 。如果数组中每个元素都不相同，则返回 false 。

// answers
impl Solution {
    pub fn p217_contains_duplicate(nums: Vec<i32>) -> bool {
        // codes
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for n in nums {
            let count = map.entry(n).or_insert(0);
            *count += 1;
            if *count > 1 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p217_contains_duplicate_t1() {
        assert_eq!(Solution::p217_contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn p217_contains_duplicate_t2() {
        assert_eq!(Solution::p217_contains_duplicate(vec![1, 2, 3, 4]), false);
    }
}
