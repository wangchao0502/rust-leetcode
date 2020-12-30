#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 有一堆石头，每块石头的重量都是正整数。
// 每一回合，从中选出两块 最重的 石头，然后将它们一起粉碎。假设石头的重量分别为 x 和 y，且 x <= y。那么粉碎的可能结果如下：
// 如果 x == y，那么两块石头都会被完全粉碎；
// 如果 x != y，那么重量为 x 的石头将会完全粉碎，而重量为 y 的石头新重量为 y-x。
// 最后，最多只会剩下一块石头。返回此石头的重量。如果没有石头剩下，就返回 0。

// answers
impl Solution {
    pub fn p1046_last_stone_weight(stones: Vec<i32>) -> i32 {
        // codes
        use std::collections::BinaryHeap;

        let mut h = BinaryHeap::from(stones);
        while h.len() > 1 {
            let d = h.pop().unwrap() - h.pop().unwrap();
            h.push(d);
        }
        h.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1046_last_stone_weight_t1() {
        assert_eq!(Solution::p1046_last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn p1046_last_stone_weight_t2() {
        assert_eq!(Solution::p1046_last_stone_weight(vec![2]), 2);
    }

    #[test]
    fn p1046_last_stone_weight_t3() {
        assert_eq!(Solution::p1046_last_stone_weight(vec![1, 2]), 1);
    }

    #[test]
    fn p1046_last_stone_weight_t4() {
        assert_eq!(Solution::p1046_last_stone_weight(vec![2, 2]), 0);
    }
}
