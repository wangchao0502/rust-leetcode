#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given n non-negative integers representing the histogram's bar height where the width of each bar is 1,
// find the area of largest rectangle in the histogram.

// answers
// 单调递增栈 + 哨兵
impl Solution {
    pub fn p84_largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // codes
        let len = heights.len();

        if len == 0 {
            return 0;
        }
        if len == 1 {
            return heights[0];
        }

        let mut ans = 0;
        // 头尾增加两个哨兵
        let mut input = vec![0; len + 2];

        for i in 0..len {
            input[i + 1] = heights[i];
        }

        let len = len + 2;
        let mut stack: Vec<usize> = vec![0];

        for i in 1..len {
            while input[i] < input[*stack.last().unwrap()] {
                let cur_height = input[stack.pop().unwrap()];
                let cur_width = (i - stack.last().unwrap() - 1) as i32;
                ans = ans.max(cur_height * cur_width);
            }
            stack.push(i);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p84_largest_rectangle_area_t1() {
        assert_eq!(
            Solution::p84_largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
            10
        );
    }
}
