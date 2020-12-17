#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
// 如果你最多只允许完成一笔交易（即买入和卖出一支股票一次），设计一个算法来计算你所能获取的最大利润。
// 注意：你不能在买入股票前卖出股票。

// answers
impl Solution {
    pub fn p121_max_profit(prices: Vec<i32>) -> i32 {
        // codes
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            if price < min_price {
                min_price = price;
            } else {
                max_profit = max_profit.max(price - min_price);
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p121_max_profit_t1() {
        assert_eq!(Solution::p121_max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
