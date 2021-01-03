#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 大餐 是指 恰好包含两道不同餐品 的一餐，其美味程度之和等于 2 的幂。
// 你可以搭配 任意 两道餐品做一顿大餐。
// 给你一个整数数组 deliciousness ，其中 deliciousness[i] 是第 i​​​​​​​​​​​​​​ 道餐品的美味程度，
// 返回你可以用数组中的餐品做出的不同 大餐 的数量。结果需要对 10^9 + 7 取余。
// 注意，只要餐品下标不同，就可以认为是不同的餐品，即便它们的美味程度相同。

// answers
impl Solution {
    pub fn p5642_count_pairs(deliciousness: Vec<i32>) -> i32 {
        // codes
        // ll ans = 0;
        // unordered_map<int, int> cnt;
        // for (int i : deliciousness)
        //     cnt[i]++;
        // for (auto [i, f] : cnt) {
        //     for (int k = 0; k <= 21; ++k) {
        //         int mask = 1 << k;
        //         int comp = mask - i;
        //         if (comp < 0)
        //             continue;
        //         if (comp != i && cnt.count(comp))
        //             ans += 1LL * f * cnt[comp];
        //         if (comp == i && f >= 2)
        //             ans += 1LL * f * (f - 1);
        //     }
        // }
        // return ans / 2 % MOD;
        use std::collections::HashMap;
        let mut cnt: HashMap<i32, i64> = HashMap::new();
        let mut ans: i64 = 0;

        for i in deliciousness {
            let count = cnt.entry(i).or_insert(0);
            *count += 1;
        }

        for (num, c) in &cnt {
            for i in 0..=21 {
                let mask = 1 << i;
                let comp = mask - num;
                if comp < 0 {
                    continue;
                }
                if let Some(cc) = cnt.get(&comp) {
                    if comp != *num && *cc > 0 {
                        ans += *c * *cc;
                    }
                }
                if comp == *num && *c >= 2 {
                    ans += *c * (*c - 1);
                }
            }
        }

        (ans / 2 % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5642_count_pairs_t1() {
        assert_eq!(Solution::p5642_count_pairs(vec![1, 1, 1, 3, 3, 3, 7]), 15);
    }

    #[test]
    fn p5642_count_pairs_t2() {
        assert_eq!(
            Solution::p5642_count_pairs(vec![
                149, 107, 1, 63, 0, 1, 6867, 1325, 5611, 2581, 39, 89, 46, 18, 12, 20, 22, 234
            ]),
            12
        );
    }
}
