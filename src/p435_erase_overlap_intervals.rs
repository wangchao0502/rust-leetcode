#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个区间的集合，找到需要移除区间的最小数量，使剩余区间互不重叠。
// 注意:
// 可以认为区间的终点总是大于它的起点。
// 区间 [1,2] 和 [2,3] 的边界相互“接触”，但没有相互重叠。

// answers
// 类似452题，最多不重叠子区间
impl Solution {
    pub fn p435_erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // codes
        let mut ans = 0;
        let mut i = 0;

        intervals.sort_unstable_by_key(|x| x[1]);

        while i < intervals.len() {
            let right = intervals[i][1];
            i += 1;
            while i < intervals.len() && intervals[i][0] < right {
                i += 1;
            }
            ans += 1;
        }

        intervals.len() as i32 - ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p435_erase_overlap_intervals_t1() {
        assert_eq!(
            Solution::p435_erase_overlap_intervals(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 3]
            ]),
            1
        );
    }

    #[test]
    fn p435_erase_overlap_intervals_t2() {
        assert_eq!(
            Solution::p435_erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }
}
