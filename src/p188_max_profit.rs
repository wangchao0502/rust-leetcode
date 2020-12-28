#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个整数数组 prices ，它的第 i 个元素 prices[i] 是一支给定的股票在第 i 天的价格。
// 设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。
// 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

// answers
impl Solution {
    pub fn p188_max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        // codes
        let k = k as usize;
        let n = prices.len();
        let k = k.min(n / 2);
        // let mut buy = vec![vec![0; k + 1]; n];
        // let mut sell = vec![vec![0; k + 1]; n];
        // buy[i][j]表示第i天买入，进行j笔交易，并且当前手上持有一支股票的最大利润
        // sell[i][j]表示第i天卖出，进行j笔交易，并且当前手上持有一支股票的最大利润
        // buy[i][j] = max(buy[i - 1][j], sell[i - 1][j] - price[i])
        // sell[i][j] = max(sell[i - 1][j], bug[i - 1][j - i] - price[i])
        // 结果为max(sell[n - 1][0..k])

        // buy[0][0] = -prices[0];
        // sell[0][0] = 0;

        // for i in 1..=k {
        //     buy[0][i] = i32::MIN / 2;
        //     sell[0][i] = i32::MIN / 2;
        // }

        // for i in 1..n {
        //     buy[i][0] = buy[i - 1][0].max(sell[i - 1][0] - prices[i]);
        //     for j in 1..=k {
        //         buy[i][j] = buy[i - 1][j].max(sell[i - 1][j] - prices[i]);
        //         sell[i][j] = sell[i - 1][j].max(buy[i - 1][j - 1] + prices[i]);
        //     }
        // }

        // *sell[n - 1].iter().max().unwrap()

        // 由于状态转移方程只用到了x[i - 1][j - 1], 所以可以优化成一维数组
        let mut buy = vec![0; k + 1];
        let mut sell = vec![0; k + 1];

        buy[0] = -prices[0];
        sell[0] = 0;

        for i in 1..=k {
            buy[i] = i32::MIN / 2;
            sell[i] = i32::MIN / 2;
        }

        for i in 1..n {
            buy[0] = buy[0].max(sell[0] - prices[i]);
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j] - prices[i]);
                sell[j] = sell[j].max(buy[j - 1] + prices[i]);
            }
        }

        *sell.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p188_max_profit_t1() {
        assert_eq!(Solution::p188_max_profit(2, vec![2, 4, 1]), 2);
    }

    #[test]
    fn p188_max_profit_t2() {
        assert_eq!(Solution::p188_max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
