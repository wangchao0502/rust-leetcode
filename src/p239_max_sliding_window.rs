#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。
// 你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
// 返回滑动窗口中的最大值。

// answers
impl Solution {
    pub fn p239_max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // codes
        use std::collections::BinaryHeap;
        // value, index
        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut ans = vec![];
        let k = k as usize;

        // init
        for i in 0..k {
            heap.push((nums[i], i));
        }

        ans.push((*heap.peek().unwrap()).0);

        // 开始滑动
        for i in k..nums.len() {
            while let Some(top) = heap.peek() {
                if (*top).1 > i - k {
                    break;
                }
                heap.pop();
            }
            heap.push((nums[i], i));
            ans.push((*heap.peek().unwrap()).0);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p239_max_sliding_window_t1() {
        assert_eq!(
            Solution::p239_max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn p239_max_sliding_window_t2() {
        assert_eq!(
            Solution::p239_max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
            vec![10, 10, 9, 2]
        );
    }

    #[test]
    fn p239_max_sliding_window_t3() {
        assert_eq!(
            Solution::p239_max_sliding_window(vec![1, -1], 1),
            vec![1, -1]
        );
    }
}
