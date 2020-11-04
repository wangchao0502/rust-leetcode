#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
// You may assume that the intervals were initially sorted according to their start times.s

// answers
impl Solution {
    pub fn p57_insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // codes
        use std::cmp::{max, min};
        let mut less = vec![];
        let mut more = vec![];
        let mut start = new_interval[0];
        let mut end = new_interval[1];

        for curr in intervals {
            if curr[1] < new_interval[0] {
                less.push(curr);
            } else if curr[0] > new_interval[1] {
                more.push(curr);
            } else {
                start = min(curr[0], start);
                end = max(curr[1], end);
            }
        }
        less.push(vec![start, end]);
        less.append(&mut more);
        less
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p57_insert_t1() {
        assert_eq!(
            Solution::p57_insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
}
