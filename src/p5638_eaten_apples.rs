#![allow(dead_code)]

// use mods
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution {}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    expired: i32,
    remain: i32,
}

// min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.expired.cmp(&self.expired)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// problem description
// 有一棵特殊的苹果树，一连 n 天，每天都可以长出若干个苹果。
// 在第 i 天，树上会长出 apples[i] 个苹果，这些苹果将会在 days[i] 天后（也就是说，第 i + days[i] 天时）腐烂，
// 变得无法食用。也可能有那么几天，树上不会长出新的苹果，此时用 apples[i] == 0 且 days[i] == 0 表示。
// 你打算每天 最多 吃一个苹果来保证营养均衡。注意，你可以在这 n 天之后继续吃苹果。
// 给你两个长度为 n 的整数数组 days 和 apples ，返回你可以吃掉的苹果的最大数目。

// answers
impl Solution {
    pub fn p5638_eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        // codes
        let n = apples.len();
        // 苹果过期列表, vec[0]:苹果个数 vec[1]:过期时间
        let mut min_heap: BinaryHeap<State> = BinaryHeap::new();
        let mut ans = 0;
        let mut i = 0;

        while i < n || !min_heap.is_empty() {
            // 1. 移除过期的
            while let Some(last) = min_heap.peek() {
                if last.expired <= i as i32 {
                    min_heap.pop();
                } else {
                    break;
                }
            }
            // 2. 添加当天的
            if i < n && apples[i] > 0 {
                min_heap.push(State {
                    expired: i as i32 + days[i],
                    remain: apples[i],
                });
            }
            // 3. 吃一个，优先吃最快过期的
            let mut need_delete = false;
            if let Some(mut last) = min_heap.peek_mut() {
                if last.remain > 0 {
                    ans += 1;
                    last.remain -= 1;
                    if last.remain == 0 {
                        need_delete = true;
                    }
                }
            }
            if need_delete {
                min_heap.pop();
            }

            i += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5638_eaten_apples_t1() {
        assert_eq!(
            // { 3: -6, 7: -5, 6: -2 }
            // [1, 3, 6, 5, 7, 7, 5, 0]
            Solution::p5638_eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]),
            7
        );
    }

    #[test]
    fn p5638_eaten_apples_t2() {
        assert_eq!(
            Solution::p5638_eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]),
            5
        );
    }

    #[test]
    fn p5638_eaten_apples_t3() {
        assert_eq!(
            Solution::p5638_eaten_apples(vec![20000], vec![20000]),
            20000
        )
    }
}
