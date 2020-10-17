#![allow(dead_code)]

// use mods
use std::collections::HashMap;

pub struct Solution {}

// add structs

// answers
// sliding window and hash map
impl Solution {
    pub fn p3_length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut start = 0;
        let mut result = 0;

        for (i, c) in s.chars().enumerate() {
            map.entry(c)
                .and_modify(|x| {
                    // start may be after x
                    start = start.max(*x + 1);
                    *x = i;
                })
                .or_insert(i);
            result = result.max(i - start + 1);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p3_length_of_longest_substring_t1() {
        assert_eq!(
            Solution::p3_length_of_longest_substring(String::from("abcabcbb")),
            3
        );
    }
}
