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
        use std::collections::HashSet;
        let mut nums = deliciousness;
        nums.sort();

        let len = nums.len();
        let mut target_set: HashSet<i64> = HashSet::new();
        let mut ans = 0;
        let mut tmp = 1;
        let mut i = 1;

        while i <= 32 {
            target_set.insert(tmp);
            tmp <<= 1;
            i += 1;
        }

        for i in 0..len {
            let mut j = i + 1;
            while j < len {
                let sum = (nums[i] + nums[j]) as i64;
                let mut skip = 1;
                if target_set.contains(&sum) {
                    ans += 1;
                    while j + skip < len && nums[j + skip] == nums[j] {
                        ans += 1;
                        skip += 1;
                    }
                }
                j += skip;
            }
        }

        ans % 1000000007
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
