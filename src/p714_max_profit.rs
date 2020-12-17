#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个整数数组 prices，其中第 i 个元素代表了第 i 天的股票价格 ；非负整数 fee 代表了交易股票的手续费用。
// 你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
// 返回获得利润的最大值。
// 注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。

// answers
// dp
impl Solution {
    pub fn p714_max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // codes
        // dp[i][0] 第i天不持有的最大利润
        // dp[i][1] 第i天持有的最大利润
        // let mut dp = vec![vec![0; 2]; prices.len()];

        // dp[0][0] = 0;
        // dp[0][1] = 0;

        // for i in 1..prices.len() {
        //     let delta = prices[i] - prices[i - 1];
        //     dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + delta - fee);
        //     dp[i][1] = dp[i - 1][0].max(dp[i - 1][1] + delta);
        // }

        // dp[prices.len() - 1][0].max(dp[prices.len() - 1][1] - fee)

        // 优化，由于每次状态转移只跟最近的两个记录有关，所以使用两个变量代替dp[i][0], dp[i][1]
        // let mut sale = 0;
        // let mut hold = 0;

        // for i in 1..prices.len() {
        //     let delta = prices[i] - prices[i - 1];
        //     let last_sale = sale;
        //     let last_hold = hold;
        //     sale = last_sale.max(last_hold + delta - fee);
        //     hold = last_sale.max(last_hold + delta);
        // }

        // sale.max(hold - fee)

        // 再优化，之前计算的价格变动，但常识是，买股票付出当前的成本，买股票赚回当前的收益，于是优化状态转移方程
        // dp[i][0]=max{dp[i−1][0],dp[i−1][1]+prices[i]−fee}
        // dp[i][1]=max{dp[i−1][1],dp[i−1][0]−prices[i]}

        let (mut cash, mut hold) = (0, -prices[0]);

        for i in 1..prices.len() {
            cash = cash.max(hold + prices[i] - fee);
            hold = hold.max(cash - prices[i]);
        }

        cash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p714_max_profit_t1() {
        assert_eq!(Solution::p714_max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }

    #[test]
    fn p714_max_profit_t2() {
        assert_eq!(
            Solution::p714_max_profit(vec![1, 4, 6, 2, 8, 3, 10, 14], 3),
            13
        );
    }
}
