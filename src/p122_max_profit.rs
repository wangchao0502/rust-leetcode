#![allow(dead_code)]

// use mods

pub struct Solution {}

// Say you have an array prices for which the ith element is the price of a given stock on day i.
// Design an algorithm to find the maximum profit. You may complete as many transactions
// as you like (i.e., buy one and sell one share of the stock multiple times).
// Note: You may not engage in multiple transactions at the same time
// (i.e., you must sell the stock before you buy again).

// answers
impl Solution {
    pub fn p122_max_profit(prices: Vec<i32>) -> i32 {
        // codes
        if prices.is_empty() {
            return 0;
        }

        let mut ans = 0;
        let mut ptr = 1;
        let mut pre = 0;

        while ptr < prices.len() {
            if prices[ptr] < prices[ptr - 1] {
                ans += prices[ptr - 1] - prices[pre];
                pre = ptr;
            }
            ptr += 1;
        }

        if ptr > pre {
            ans += prices[ptr - 1] - prices[pre];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p122_max_profit_t1() {
        assert_eq!(Solution::p122_max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn p122_max_profit_t2() {
        assert_eq!(Solution::p122_max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn p122_max_profit_t3() {
        assert_eq!(Solution::p122_max_profit(vec![5, 4, 3, 2, 1]), 0);
    }
}
