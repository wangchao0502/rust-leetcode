#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个正整数数组 nums ，请你从中删除一个含有 若干不同元素 的子数组。删除子数组的 得分 就是子数组各元素之 和 。
// 返回 只删除一个 子数组可获得的 最大得分 。
// 如果数组 b 是数组 a 的一个连续子序列，即如果它等于 a[l],a[l+1],...,a[r] ，那么它就是 a 的一个子数组。

// answers
impl Solution {
    pub fn p5630_maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        // codes
        use std::collections::HashSet;
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut set = HashSet::new();
        let mut ans = 0;

        for n in nums {
            queue.push_back(n);
            if set.contains(&n) {
                // 已经存在，出队
                while *queue.front().unwrap() != n {
                    let x = queue.pop_front().unwrap();
                    set.remove(&x);
                }
                // remove first n
                queue.pop_front();
            } else {
                ans = ans.max(queue.iter().sum());
            }
            set.insert(n);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5630_maximum_unique_subarray_t1() {
        assert_eq!(
            Solution::p5630_maximum_unique_subarray(vec![4, 2, 4, 5, 6]),
            17
        );
    }

    #[test]
    fn p5630_maximum_unique_subarray_t2() {
        assert_eq!(
            Solution::p5630_maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
            8
        );
    }

    #[test]
    fn p5630_maximum_unique_subarray_t3() {
        assert_eq!(
            Solution::p5630_maximum_unique_subarray(vec![
                187, 470, 25, 436, 538, 809, 441, 167, 477, 110, 275, 133, 666, 345, 411, 459, 490,
                266, 987, 965, 429, 166, 809, 340, 467, 318, 125, 165, 809, 610, 31, 585, 970, 306,
                42, 189, 169, 743, 78, 810, 70, 382, 367, 490, 787, 670, 476, 278, 775, 673, 299,
                19, 893, 817, 971, 458, 409, 886, 434
            ]),
            16911
        );
    }
}
