#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。
// 你需要按照以下要求，帮助老师给这些孩子分发糖果：
// 每个孩子至少分配到 1 个糖果。
// 相邻的孩子中，评分高的孩子必须获得更多的糖果。
// 那么这样下来，老师至少需要准备多少颗糖果呢？

// answers
// 1. 两次左右遍历，取每次最小
impl Solution {
    pub fn p135_candy(ratings: Vec<i32>) -> i32 {
        // codes
        let len = ratings.len();
        let mut ans = 1;
        let mut inc = 1;
        let mut dec = 0;
        let mut pre = 1;

        for i in 1..len {
            if ratings[i] >= ratings[i - 1] {
                dec = 0;
                pre = if ratings[i] == ratings[i - 1] {
                    1
                } else {
                    pre + 1
                };
                ans += pre;
                inc = pre;
            } else {
                dec += 1;
                if dec == inc {
                    dec += 1;
                }
                ans += dec;
                pre = 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p135_candy_t1() {
        assert_eq!(Solution::p135_candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn p135_candy_t2() {
        assert_eq!(Solution::p135_candy(vec![1, 2, 2]), 4);
    }
}
