#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs
// Given an array of integers arr, write a function that returns true if and
// only if the number of occurrences of each value in the array is unique.

// answers
impl Solution {
    pub fn p1207_unique_occurrences(arr: Vec<i32>) -> bool {
        // codes
        use std::collections::{HashMap, HashSet};

        let mut map = HashMap::new();
        let mut count_set: HashSet<i32> = HashSet::new();

        for i in &arr {
            let counter = map.entry(i).or_insert(0);
            *counter += 1;
        }

        for (_, &v) in map.iter() {
            count_set.insert(v);
        }

        count_set.len() == map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1207_unique_occurrences_t1() {
        assert_eq!(
            Solution::p1207_unique_occurrences(vec![1, 2, 2, 1, 1, 3]),
            true
        );
    }
    #[test]
    fn p1207_unique_occurrences_t2() {
        assert_eq!(Solution::p1207_unique_occurrences(vec![1, 2]), false);
    }
}
