#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个整数数组 nums，请你返回其中位数为 偶数 的数字的个数。

// answers
// 数学：一个数字的位数等于log10向下取整 + 1
impl Solution {
    pub fn p1295_find_numbers(nums: Vec<i32>) -> i32 {
        // codes
        nums.into_iter()
            .filter(|x| ((*x as f64).log10() as i32 + 1) % 2 == 0)
            .collect::<Vec<i32>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1295_find_numbers_t1() {
        assert_eq!(Solution::p1295_find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    }

    #[test]
    fn p1295_find_numbers_t2() {
        assert_eq!(Solution::p1295_find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
